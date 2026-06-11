use crate::composer::{compose_tweets, Alert, ResolvedTransfer, TransferEvent};
use bigdecimal::ToPrimitive;
use consts::{Token, ADDRESS_LIST, TOKENS};
use dotenv::dotenv;
use log::{error, info};
use num_bigint::BigUint;
use reqwest::Url;
use starknet_rust::{
    core::types::{EmittedEvent, Felt},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::{collections::HashMap, error::Error};
use std::{
    env,
    fs::{self, OpenOptions},
};

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod composer;
mod consts;
mod db;
mod logger;
mod starknet_id;
mod twitter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    check_db();
    check_valid_env();
    let rpc_client = get_infura_client();
    let last_network_block = rpc_client.block_number().await? - 3;

    let last_processed_block = db::get_last_processed_block().await;
    info!("Start {}", last_processed_block);
    if last_processed_block >= last_network_block {
        info!(
            "No block to process {} >= {}\n",
            last_processed_block, last_network_block
        );
        return Ok(());
    }

    if last_network_block > last_processed_block + 500 {
        info!(
            "Local is {} blocks behind (current: {}). To sync use option -s",
            last_network_block - last_processed_block,
            last_network_block
        );
        let args: Vec<String> = env::args().collect();
        if args.contains(&String::from("-s")) || last_network_block > last_processed_block + 5000 {
            info!("Updating to latest block...\n");
            db::set_last_processed_block(last_network_block).await;
        }
        return Ok(());
    }

    for token in TOKENS {
        let events_by_tx =
            get_events_to_tweet_about(token, &rpc_client, last_processed_block, last_network_block)
                .await;
        if events_by_tx.is_empty() {
            continue;
        }

        // Gather the I/O once per token: the Rate and the resolved names, then hand a pure
        // Alert to the Composer.
        let rate = fetch_rate(token).await;
        let mut name_cache: HashMap<Felt, String> = HashMap::new();

        for (tx_hash, events) in events_by_tx {
            let mut transfers = Vec::with_capacity(events.len());
            for event in events {
                let transfer = TransferEvent::from(event);
                let from_name = resolve_party(&rpc_client, transfer.from, &mut name_cache).await;
                let to_name = resolve_party(&rpc_client, transfer.to, &mut name_cache).await;
                transfers.push(ResolvedTransfer {
                    from: transfer.from,
                    to: transfer.to,
                    from_name,
                    to_name,
                    amount: transfer.amount,
                });
            }

            let alert = Alert {
                token,
                tx: tx_hash,
                rate: rate.clone(),
                transfers,
            };
            let tweets = compose_tweets(&alert);
            if tweets.len() > 1 {
                info!("Multicall transfer split into {} tweets", tweets.len());
            }
            for tweet in tweets {
                twitter::tweet(tweet).await;
            }
        }
    }

    db::set_last_processed_block(last_network_block + 1).await;
    info!("End {}\n", last_network_block + 1);
    Ok(())
}

async fn get_events_to_tweet_about(
    token: &Token,
    rpc_client: &JsonRpcClient<HttpTransport>,
    from_block: u64,
    to_block: u64,
) -> HashMap<Felt, Vec<EmittedEvent>> {
    let events = match api::fetch_events(rpc_client, token, from_block, to_block).await {
        Ok(events) => events,
        Err(e) => {
            error!(
                "fetch_events failed for {} (blocks {}-{}): {:?}",
                token.symbol, from_block, to_block, e
            );
            panic!("fetch_events failed for {}: {:?}", token.symbol, e);
        }
    };
    info!("{} Transfer for {}", events.len(), token.symbol);
    let threshold = to_u256(10_u128.pow(token.decimals.into()) * token.threshold, 0);
    let mut events_by_tx: HashMap<Felt, Vec<EmittedEvent>> = HashMap::new();

    // Filter events and group them by transaction hash
    for event in events.into_iter().filter(|event| {
        // ERC20 Transfer
        let (low, high) = if event.data.len() == 2 {
            (event.data[0], event.data[1])
        } else {
            (event.data[2], event.data[3])
        };

        to_u256(low.try_into().unwrap(), high.try_into().unwrap()) > threshold
    }) {
        events_by_tx
            .entry(event.transaction_hash)
            .or_default()
            .push(event);
    }

    events_by_tx
}

/// The token's USD Rate, fetched once per run. `None` when the token has no `rate_api_id`.
async fn fetch_rate(token: &Token) -> Option<BigUint> {
    match token.rate_api_id {
        Some(coin_id) => {
            let rate = api::fetch_coin(coin_id).await.unwrap();
            Some(BigUint::new(vec![(rate * 10000_f64).to_u32().unwrap()]))
        }
        None => None,
    }
}

/// Resolve one address to a Party name, memoised per run. The zero address is a Bridge end
/// and is left empty — the Composer renders it without ever naming it.
async fn resolve_party(
    rpc_client: &JsonRpcClient<HttpTransport>,
    address: Felt,
    cache: &mut HashMap<Felt, String>,
) -> String {
    if address == Felt::ZERO {
        return String::new();
    }
    if let Some(name) = cache.get(&address) {
        return name.clone();
    }
    let name = resolve_uncached(rpc_client, address).await;
    cache.insert(address, name.clone());
    name
}

async fn resolve_uncached(rpc_client: &JsonRpcClient<HttpTransport>, address: Felt) -> String {
    let address_as_hex = format!("{:#x}", address);
    if let Some(known) = ADDRESS_LIST
        .iter()
        .find(|item| address_as_hex.ends_with(item.address))
    {
        return known.name.to_string();
    }
    match starknet_id::address_to_domain(rpc_client, address).await {
        Some(name) => name,
        None => format!(
            "{}...{}",
            &address_as_hex[0..5],
            &address_as_hex[address_as_hex.len() - 4..],
        ),
    }
}

fn check_valid_env() {
    dotenv().ok();
    std::env::var("COIN_MARKET_CAP_API_KEY").expect("COIN_MARKET_CAP_API_KEY must be set.");
    std::env::var("NODE_PROVIDER_API_KEY").expect("NODE_PROVIDER_API_KEY must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_ID").expect("TWITTER_OAUTH2_CLIENT_ID must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_SECRET")
        .expect("TWITTER_OAUTH2_CLIENT_SECRET must be set.");
}

fn check_db() {
    fs::create_dir_all("./db").expect("Couldn't create db folder");
    OpenOptions::new()
        .write(true)
        .create(true)
        .open("./db/token.json")
        .expect("Couldn't create token file");
}

pub fn get_infura_client() -> JsonRpcClient<HttpTransport> {
    let api_key = dotenv!("NODE_PROVIDER_API_KEY");
    let rpc_url =
        format!("https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_10/{api_key}");
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

fn to_u256(low: u128, high: u128) -> BigUint {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend(high.to_be_bytes());
    bytes.extend(low.to_be_bytes());

    BigUint::from_bytes_be(&bytes[..])
}
#[cfg(test)]
mod tests {
    use super::to_u256;
    use num_bigint::BigUint;

    #[test]
    fn test_big_int() {
        let u256_0 = to_u256(5456465465465465412, 11);
        let u256_1 = to_u256(5456465465465465412, 12);
        let u256_2 = to_u256(5456465465465465413, 12);
        assert!(u256_0 < u256_1, "0");
        assert!(u256_1 < u256_2, "1");
        assert!((u256_1 + BigUint::new(vec![1])).eq(&u256_2), "2");
    }
}
