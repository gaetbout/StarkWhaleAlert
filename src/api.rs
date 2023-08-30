use dotenv::dotenv;
use reqwest::header::HeaderValue;
use reqwest::{header, Url};
use serde::{Deserialize, Serialize};
use starknet::core::types::{EventFilter, BlockId, FieldElement};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::Provider;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient};
use std::time::Duration;

use crate::{COINCAP_API_KEY, NODE_PROVIDER_API_KEY};

#[derive(Debug, Serialize, Deserialize)]
struct FetchCoinResponse {
    data: Data,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: String,
    rank: String,
    symbol: String,
    name: String,
    supply: String,
    #[serde(rename = "maxSupply")]
    max_supply: Option<String>,
    #[serde(rename = "marketCapUsd")]
    market_cap_usd: String,
    #[serde(rename = "volumeUsd24Hr")]
    volume_usd_24_hr: String,
    #[serde(rename = "priceUsd")]
    price_usd: String,
    #[serde(rename = "changePercent24Hr")]
    change_percent_24_hr: String,
    #[serde(rename = "vwap24Hr")]
    vwap_24_hr: String,
}

pub async fn fetch_coin(coin_id: &str) -> Result<f64, reqwest::Error> {
    dotenv().ok();
    let token = std::env::var(COINCAP_API_KEY).expect("COINCAP_API_KEY must be set");

    let mut headers = reqwest::header::HeaderMap::new();
    let auth = String::from(format!("Bearer {token}"));
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::try_from(auth).expect("Unable to parse the Bearer token"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed building the client");

    let get_link: String = format!("{}{}", "https://api.coincap.io/v2/assets/", coin_id);

    let coin_info: FetchCoinResponse = client
        .get(get_link)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json()
        .await?;

    println!("{}:{}", coin_info.data.name, coin_info.data.price_usd);
    Ok(coin_info.data.price_usd.parse().unwrap())
}

pub async fn fetch_events() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let token = std::env::var(NODE_PROVIDER_API_KEY).expect("COINCAP_API_KEY must be set");

    let rpc_url = format!("https://starknet-mainnet.infura.io/v3/{token}");
    let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));

    let events = rpc_client
        .get_events(
            EventFilter {
                from_block: Some(BlockId::Number(181710)),
                to_block: Some(BlockId::Number(181712)),
                address: Some(FieldElement::from_hex_be("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap()),
                keys: Some(vec![vec![get_selector_from_name("Transfer").unwrap()]]),
            },
            None,
            20,
        )
        .await
        .unwrap();

        println!("{:?}", events.events[0]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{fetch_coin, fetch_events};
    use rstest::rstest;

    #[rstest]
    #[case("ethereum")]
    #[case("usd-coin")]
    #[case("tether")]
    #[tokio::test]
    async fn test_fetch_coin(#[case] coin: &str) {
        let value = fetch_coin(coin).await.unwrap();

        println!("Value is {}", value);
    }

    #[tokio::test]
    async fn test_fetch_events() {
        fetch_events().await.unwrap();
    }
}
