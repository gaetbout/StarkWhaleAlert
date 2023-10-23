use reqwest::{header, header::HeaderValue};
use serde::{Deserialize, Serialize};
use starknet::{
    core::{
        types::{BlockId, EmittedEvent, EventFilter, FieldElement},
        utils::get_selector_from_name,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::time::Duration;

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
    pub address: &'static str,
    pub decimals: u8,
    pub symbol: &'static str,
    pub selector: &'static str, // This should be the String of the selector (Transfer, ...), not the HEX value
    pub threshold: u128,
    pub logo: &'static str,
    pub rate_api_id: &'static str,
}

pub async fn fetch_coin(coin_id: &str) -> Result<f64, reqwest::Error> {
    let token = dotenv!("COINCAP_API_KEY");

    let mut headers = reqwest::header::HeaderMap::new();
    let auth = format!("Bearer {token}");
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

    Ok(coin_info.data.price_usd.parse().expect("Error: fetch_coin"))
}

// TODO check what can be impl on the token object
pub async fn fetch_events(
    rpc_client: &JsonRpcClient<HttpTransport>,
    token: &Token,
    from_block: u64,
    to_block: u64,
) -> Result<Vec<EmittedEvent>, reqwest::Error> {
    let mut events = vec![];
    let mut continuation_token = None;
    let from_block = Some(BlockId::Number(from_block));
    let to_block = Some(BlockId::Number(to_block));
    let address = Some(FieldElement::from_hex_be(token.address).expect("Invalid address"));
    let keys = Some(vec![vec![
        get_selector_from_name(token.selector).expect("Invalid selector")
    ]]);

    loop {
        let event_page = rpc_client
            .get_events(
                EventFilter {
                    from_block,
                    to_block,
                    address,
                    keys: keys.clone(),
                },
                continuation_token,
                1000,
            )
            .await
            .expect("Error: fetch_events");

        events.extend(event_page.events);
        match event_page.continuation_token {
            Some(_) => continuation_token = event_page.continuation_token,
            None => return Ok(events),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_infura_client;

    use super::{fetch_coin, fetch_events, Token};
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
        let eth = Token {
            address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",

            decimals: 18,
            symbol: "ETH",
            selector: "Transfer",
            threshold: 50, // 50 eth
            logo: "♦",
            rate_api_id: "ethereum",
        };
        fetch_events(&get_infura_client(), &eth, 200000, 200001)
            .await
            .unwrap();
    }
}
