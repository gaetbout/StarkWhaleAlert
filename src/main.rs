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

use crate::consts::TOKENS;

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod consts;
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

        for emitted_event in to_tweet {
            let text_to_tweet = formatter::get_formatted_text(emitted_event, token).await;
            twitter::tweet(text_to_tweet).await;
        }
    }

    // twitter::tweet("LAMA".to_string()).await;

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
    std::env::var("COINCAP_API_KEY").expect("COINCAP_API_KEY must be set.");
    std::env::var("NODE_PROVIDER_API_KEY").expect("NODE_PROVIDER_API_KEY must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_ID").expect("TWITTER_OAUTH2_CLIENT_ID must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_SECRET")
        .expect("TWITTER_OAUTH2_CLIENT_SECRET must be set.");
}

pub fn get_infura_client() -> JsonRpcClient<HttpTransport> {
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
