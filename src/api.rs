use dotenv::dotenv;
use reqwest::header;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::COINCAP_API_KEY;

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    data: Inner,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Inner {
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

pub async fn fetch_coin(coin_id: &str) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let mut headers = reqwest::header::HeaderMap::new();
    let token = std::env::var(COINCAP_API_KEY).expect("COINCAP_API_KEY must be set");
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

    let coin_info: Root = client
        .get(get_link)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json()
        .await?;

    println!("{:}", coin_info.data.price_usd);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::fetch_coin;
    use rstest::rstest;

    #[rstest]
    #[case("ethereum")]
    #[case("usd-coin")]
    #[case("tether")]
    #[tokio::test]
    async fn it_works(#[case] coin: &str) {
        fetch_coin(coin).await.unwrap();
    }
}
