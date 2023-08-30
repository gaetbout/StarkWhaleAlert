use dotenv::dotenv;
use reqwest::header::HeaderValue;
use reqwest::{header, Url};
use serde::{Deserialize, Serialize};
use starknet::core::types::{BlockId, EventFilter, FieldElement};
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

pub struct Token {
    address: String,
    decimals: u8,
    symbol: String,
    selector: String, // This should be the String of the selector (Transfer, ...), not the HEX value
    threshold: FieldElement,  // TODO Could maybe take just how much and use decimals to reach total?
    logo: String,
    rate_api_id: String,
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

pub async fn fetch_events(token: Token) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let api_key = std::env::var(NODE_PROVIDER_API_KEY).expect("COINCAP_API_KEY must be set");

    let rpc_url = format!("https://starknet-mainnet.infura.io/v3/{api_key}");
    let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));

    let events = rpc_client
        .get_events(
            EventFilter {
                from_block: Some(BlockId::Number(181710)),
                to_block: Some(BlockId::Number(181711)),
                address: Some(FieldElement::from_hex_be(token.address.as_str()).unwrap()),
                keys: Some(vec![vec![
                    get_selector_from_name(token.selector.as_str()).unwrap()
                ]]),
            },
            Some("1000".to_string()),
            1000,
        )
        .await
        .unwrap();

    println!("{:?}", events.events.len());
    println!(
        "Cont token: {:?}",
        events
            .continuation_token
            .unwrap_or("No cont token".to_string())
    );
    println!("decimals: {:?}", token.decimals);
    println!("threshold: {:?}", token.threshold);
    let filtered_events:Vec<_> = events
        .events
        .iter()
        .filter(|event| event.data[2] > token.threshold).collect();
    // TODO Handle the data part 2?
    println!("{:?}", filtered_events);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{fetch_coin, fetch_events, Token};
    use rstest::rstest;
    use starknet::core::types::FieldElement;

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
        let eth = Token {
            address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                .to_string(),
            decimals: 18,
            symbol: "ETH".to_string(),
            selector: "Transfer".to_string(),
            threshold: FieldElement::from(10_u128.pow(18)  * 50), // 50 eth
            logo: "♦".to_string(),
            rate_api_id: "ethereum".to_string(),
        };
        fetch_events(eth).await.unwrap();
    }
}
