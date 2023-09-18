use api::{fetch_events, Token};
use dotenv::dotenv;
use log::info;
use num_bigint::{BigUint, ToBigInt};
use reqwest::Url;
use starknet::{
    core::types::EmittedEvent,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::error::Error;

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod db;
mod formatter;
mod logger;
mod twitter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    info!("Start");
    check_valid_env();
    let rpc_client = get_infura_client();
    let last_block = rpc_client
        .block_number()
        .await
        .expect("Error while getting last block")
        - 1;
    info!("Current number: {}", last_block);

    for token in TOKENS {
        // Prob a better way to do, like spawning a thread to do all this in parrallel?
        let to_tweet = get_events_to_tweet_about(token, &rpc_client, last_block).await;
        println!("TO TWEET {:?}", to_tweet);
        // to_tweet.iter().fo
    }
    // TODO Const file?
    // twitter::tweet("Someteaeazzhing".to_string()).await;
    db::set_last_processsed_block(None, last_block).await;
    info!("End");

    Ok(())
}

async fn get_events_to_tweet_about(
    token: &Token,
    rpc_client: &JsonRpcClient<HttpTransport>,
    last_block: u64,
) -> Vec<EmittedEvent> {
    let events = fetch_events(&rpc_client, token, last_block - 5, last_block)
        .await
        .unwrap();

    let threshold = to_u256(10_u128.pow(token.decimals.into()) * token.threshold, 0);
    let filtered_events: Vec<_> = events
        .into_iter()
        .filter(|event| {
            let low: u128 = event.data[2].try_into().unwrap();
            let high = event.data[3].try_into().unwrap();

            to_u256(low, high) > threshold
        })
        .collect();
    filtered_events.clone()
}

fn check_valid_env() {
    dotenv().ok();
    std::env::var("COINCAP_API_KEY").expect("COINCAP_API_KEY must be set.");
    std::env::var("NODE_PROVIDER_API_KEY").expect("NODE_PROVIDER_API_KEY must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_ID").expect("TWITTER_OAUTH2_CLIENT_ID must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_SECRET")
        .expect("TWITTER_OAUTH2_CLIENT_SECRET must be set.");
}

fn get_infura_client() -> JsonRpcClient<HttpTransport> {
    let api_key = dotenv!("NODE_PROVIDER_API_KEY");
    let rpc_url = format!("https://starknet-mainnet.infura.io/v3/{api_key}");
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

fn to_u256(low: u128, high: u128) -> BigUint {
    // There is prob a better solution to do that...
    let mut low_vec = low.to_bigint().unwrap().to_u32_digits().1;
    let mut high_vec = high.to_bigint().unwrap().to_u32_digits().1;
    for _ in low_vec.len()..4 {
        low_vec.push(0_u32)
    }
    low_vec.append(&mut high_vec);
    BigUint::new(low_vec)
}

const ETH: Token = Token {
    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
    selector: "Transfer",
    threshold: 50,
    logo: "♦",
    rate_api_id: "ethereum",
};

const USDC: Token = Token {
    address: "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
    decimals: 6,
    symbol: "USDC",
    selector: "Transfer",
    threshold: 60_000,
    logo: "$",
    rate_api_id: "usd-coin",
};

const USDT: Token = Token {
    address: "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8",
    decimals: 6,
    symbol: "USDT",
    selector: "Transfer",
    threshold: 60_000,
    logo: "$",
    rate_api_id: "tether",
};

pub struct AddressToName {
    address: &'static str,
    name: &'static str,
}

const LAYER_SWAP: AddressToName = AddressToName {
    address: "19252b1deef483477c4d30cfcc3e5ed9c82fafea44669c182a45a01b4fdb97a",
    name: "Layerswap",
};

const ZKLEND_MARKET: AddressToName = AddressToName {
    address: "4c0a5193d58f74fbace4b74dcf65481e734ed1714121bdc571da345540efa05",
    name: "zkLend: Market",
};

const BRIQ_FACTORY: AddressToName = AddressToName {
    address: "5b021b6743c4f420e20786baa7fb9add1d711302c267afbc171252a74687376",
    name: "The Fucking Briq",
};

const STARKNET_DEPLOYER: AddressToName = AddressToName {
    address: "1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
    name: "Starknet deployer",
};

const MY_SWAP_AMM: AddressToName = AddressToName {
    address: "10884171baf1914edc28d7afb619b40a4051cfae78a094a55d230f19e944a28",
    name: "mySwap: AMM Swap",
};

const JEDI_SWAP_ETH_USDC: AddressToName = AddressToName {
    address: "4d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a",
    name: "JediSwap: ETH/USDC Pair",
};

const TENK_SWAP_ETH_USD: AddressToName = AddressToName {
    address: "23c72abdf49dffc85ae3ede714f2168ad384cc67d08524732acea90df325",
    name: "10KSwap: ETH-USDC Pair",
};

const ORBITER_FINANCE_BRIDGE_1: AddressToName = AddressToName {
    address: "7b393627bd514d2aa4c83e9f0c468939df15ea3c29980cd8e7be3ec847795f0",
    name: "Orbiter Finance Bridge 1",
};

const ORBITER_FINANCE_BRIDGE_4: AddressToName = AddressToName {
    address: "6e18dd81378fd5240704204bccc546f6dfad3d08c4a3a44347bd274659ff328",
    name: "Orbiter Finance Bridge 4",
};

const ORBITER_FINANCE_BRIDGE_2: AddressToName = AddressToName {
    address: "64a24243f2aabae8d2148fa878276e6e6e452e3941b417f3c33b1649ea83e11",
    name: "Orbiter Finance Bridge 2",
};

const TOKENS: &'static [Token] = &[ETH, USDC, USDT];
const address_list: &'static [AddressToName] = &[
    LAYER_SWAP,
    ZKLEND_MARKET,
    BRIQ_FACTORY,
    STARKNET_DEPLOYER,
    MY_SWAP_AMM,
    JEDI_SWAP_ETH_USDC,
    TENK_SWAP_ETH_USD,
    ORBITER_FINANCE_BRIDGE_1,
    ORBITER_FINANCE_BRIDGE_2,
    ORBITER_FINANCE_BRIDGE_4,
];

fn ends_with(a: &str) -> Option<&AddressToName> {
    address_list.iter().find(|item| a.ends_with(item.address))
}
#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::{ends_with, to_u256};

    #[test]
    fn test_big_int() {
        let u256_0 = to_u256(5456465465465465412, 11);
        let u256_1 = to_u256(5456465465465465412, 12);
        let u256_2 = to_u256(5456465465465465413, 12);
        assert!(u256_0 < u256_1, "0");
        assert!(u256_1 < u256_2, "1");
        assert!((u256_1 + BigUint::new(vec![1])).eq(&u256_2), "2");
    }

    #[test]
    fn test_ends_with() {
        let a = ends_with("0x7b393627bd514d2aa4c83e9f0c468939df15ea3c29980cd8e7be3ec847795f0");
        assert!(a.is_some(), "Should be some");
        assert!(
            a.unwrap().name == "Orbiter Finance Bridge 1",
            "Should be Orbiter Finance Bridge 1"
        );

        let b = ends_with("0x7b393627bd5132114c83e9f0c468939df15ea3c29980cd8e7be3ec847795f0");
        assert!(b.is_none(), "Should be None");
    }
}
