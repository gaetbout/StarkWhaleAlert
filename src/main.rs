use api::Token;
use dotenv::dotenv;
use log::info;
use num_bigint::{BigUint, ToBigInt};
use reqwest::Url;
use starknet::{
    core::types::FieldElement,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::error::Error;

use crate::api::fetch_events;
#[macro_use]
extern crate dotenv_codegen;

mod api;
mod db;
mod logger;
mod twitter;

const ETH: Token = Token {
    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
    selector: "Transfer",
    threshold: 50, //FieldElement::from(10_u128.pow(18) * 50), // 50 eth
    logo: "♦",
    rate_api_id: "ethereum",
};

const TOKENS: &'static [Token] = &[ETH];

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
    info!(
        "Last proccessed number: {}",
        db::get_last_processed_block(None).await
    );

    let token = &TOKENS[0];
    let events = fetch_events(rpc_client, token, last_block - 5, last_block - 1)
        .await
        .unwrap();

    let threshold = FieldElement::from((10_u128.pow(token.decimals.into())) * token.threshold);
    let filtered_events: Vec<_> = events
        .iter()
        .filter(|event| event.data[2] > threshold)
        .collect();
    println!("Filtered events: {:?}", filtered_events);
    // twitter::tweet("Someteaeazzhing".to_string()).await;
    db::set_last_processsed_block(None, last_block).await;
    info!("End");

    Ok(())
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
    let mut low_vec = low.to_bigint().unwrap().to_u32_digits().1;
    let mut high_vec = high.to_bigint().unwrap().to_u32_digits().1;
    for _ in low_vec.len()..4 {
        low_vec.push(0_u32)
    }
    low_vec.append(&mut high_vec);
    BigUint::new(low_vec)
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
