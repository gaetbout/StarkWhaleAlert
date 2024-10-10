use crate::consts::Token;
use serde::{Deserialize, Serialize};
use starknet::{
    core::{
        types::{BlockId, EmittedEvent, EventFilter, Felt},
        utils::get_selector_from_name,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError},
};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct FetchCoinResponse {
    data: HashMap<String, Data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    quote: HashMap<String, Quote>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    price: f64,
}

pub async fn fetch_coin(coin_id: &str) -> Result<f64, reqwest::Error> {
    let token = dotenv!("COIN_MARKET_CAP_API_KEY");

    let client = reqwest::Client::builder().build()?;

    let coin_info: FetchCoinResponse = client
        .get("https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest")
        .header("Accepts", "application/json")
        .header("X-CMC_PRO_API_KEY", token)
        .query(&[("slug", coin_id)])
        .query(&[("convert", "USD")])
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json()
        .await?;

    Ok(coin_info
        .data
        .values()
        .into_iter()
        .next()
        .expect("fetch_coin: data map issue")
        .quote
        .get("USD")
        .expect("fetch_coin: quote map issue")
        .price)
}

pub async fn fetch_events(
    rpc_client: &JsonRpcClient<HttpTransport>,
    token: &Token,
    from_block: u64,
    to_block: u64,
) -> Result<Vec<EmittedEvent>, ProviderError> {
    let mut events = vec![];
    let mut continuation_token = None;
    let from_block = Some(BlockId::Number(from_block));
    let to_block = Some(BlockId::Number(to_block));
    let address = Some(Felt::from_hex(token.address).expect("Invalid address"));
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
            .await?;

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
    #[case("multi-collateral-dai")]
    #[case("starknet-token")]
    #[case("wrapped-bitcoin")]
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
            rate_api_id: Some("ethereum"),
        };
        fetch_events(&get_infura_client(), &eth, 200000, 200001)
            .await
            .unwrap();
    }
}
