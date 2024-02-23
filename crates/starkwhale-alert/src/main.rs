use consts::{Token, TOKENS};
use dotenv::dotenv;
use log::info;
use num_bigint::BigUint;
use reqwest::Url;
use starknet::{
    core::types::EmittedEvent,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::error::Error;
use std::{
    env,
    fs::{self, OpenOptions},
};

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod consts;
mod db;
mod formatter;
mod logger;
mod starknet_id;
mod twitter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    check_db();
    check_valid_env();
    let rpc_client = get_infura_client();
    let last_network_block = rpc_client.block_number().await? - 1;

    let last_processed_block = db::get_last_processed_block().await;
    info!("Start {}", last_processed_block);
    if last_processed_block >= last_network_block {
        info!(
            "No block to process {} >= {}\n",
            last_processed_block, last_network_block
        );
        return Ok(());
    }

    if last_network_block > last_processed_block + 100 {
        // TODO Send mail
        info!(
            "Local is {} blocks behind (current: {}). To sync use option -s",
            last_network_block - last_processed_block,
            last_network_block
        );
        let args: Vec<String> = env::args().collect();
        if args.contains(&String::from("-s")) {
            info!("Updating to latest block...\n");
            db::set_last_processed_block(last_network_block).await;
        }
        return Ok(());
    }

    for token in TOKENS {
        // TODO Prob a better way to do, like spawning a thread and do each in parrallel?
        let to_tweet =
            get_events_to_tweet_about(token, &rpc_client, last_processed_block, last_network_block)
                .await;

        for emitted_event in to_tweet {
            let text_to_tweet = formatter::get_formatted_text(emitted_event, token).await;
            twitter::tweet(text_to_tweet).await;
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
) -> Vec<EmittedEvent> {
    let events = api::fetch_events(rpc_client, token, from_block, to_block)
        .await
        .unwrap();

    info!("{} Transfer for {}", events.len(), token.symbol);
    let threshold = to_u256(10_u128.pow(token.decimals.into()) * token.threshold, 0);
    events
        .into_iter()
        .filter(|event| {
            let low: u128 = event.data[2].try_into().unwrap();
            let high = event.data[3].try_into().unwrap();

            to_u256(low, high) > threshold
        })
        .collect()
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
    let rpc_url = format!("https://starknet-mainnet.infura.io/v3/{api_key}");
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
    use num_bigint::BigUint;

    use super::to_u256;

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
