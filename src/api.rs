use bigdecimal::num_traits;
use dotenv::dotenv;
use num_traits::cast::ToPrimitive;
use reqwest::header::HeaderValue;
use reqwest::{header, Url};
use serde::{Deserialize, Serialize};
use starknet::core::types::{BlockId, BlockTag, EventFilter, FieldElement, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::Provider;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient};
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
    address: String,
    decimals: u8,
    symbol: String,
    selector: String, // This should be the String of the selector (Transfer, ...), not the HEX value
    threshold: FieldElement, // TODO Could maybe take just how much and use decimals to reach total?
    logo: String,
    rate_api_id: String,
}

pub async fn fetch_coin(coin_id: &str) -> Result<f64, reqwest::Error> {
    let token = dotenv!("COINCAP_API_KEY");

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
    let api_key = dotenv!("NODE_PROVIDER_API_KEY");

    let rpc_url = format!("https://starknet-mainnet.infura.io/v3/{api_key}");
    let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));

    let bn = rpc_client.block_number().await.unwrap();
    println!("Block number {}", bn);

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
    let filtered_events: Vec<_> = events
        .events
        .iter()
        .filter(|event| event.data[2] > token.threshold)
        .collect();
    // TODO Handle the data part 2?
    println!("{:?}", filtered_events);

    Ok(())
}

async fn address_to_domain(address: FieldElement, contract_addr: FieldElement) {
    let api_key = dotenv!("NODE_PROVIDER_API_KEY");
    let rpc_url = format!("https://starknet-mainnet.infura.io/v3/{api_key}");
    let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));
    let repsonse = rpc_client
        .call(
            FunctionCall {
                contract_address: contract_addr,
                entry_point_selector: get_selector_from_name("address_to_domain").unwrap(),
                calldata: vec![address],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    let mut domain = String::new();
    repsonse.iter().skip(1).for_each(|value| {
        domain.push_str(decode(*value).as_str());
        domain.push('.');
    });
    domain.push_str("stark");
    println!("DOMAIN FOUND {}", domain);
}

const BASIC_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789-";
const BIG_ALPHABET: &str = "这来";

pub fn decode(mut felt: FieldElement) -> String {
    let mut decoded: String = String::new();
    let basic_plus = FieldElement::from(BASIC_ALPHABET.chars().count() + 1);
    let basic_len = FieldElement::from(BASIC_ALPHABET.chars().count());
    let big_plus = FieldElement::from(BIG_ALPHABET.chars().count() + 1);
    let big_len = FieldElement::from(BIG_ALPHABET.chars().count());
    let last_big = BIG_ALPHABET.chars().last().unwrap();
    while felt != FieldElement::ZERO {
        let code = felt % basic_plus;
        felt = felt.floor_div(basic_plus);
        if code == basic_len {
            let next_felt = felt.floor_div(big_plus);
            if next_felt == FieldElement::ZERO {
                let code2 = felt % big_plus;
                felt = next_felt;
                decoded.push(if code2 == FieldElement::ZERO {
                    BASIC_ALPHABET.chars().next().unwrap()
                } else {
                    last_big
                });
            } else {
                decoded.push(
                    BIG_ALPHABET
                        .chars()
                        .nth((felt % big_len).to_big_decimal(0).to_usize().unwrap())
                        .unwrap(),
                );
                felt = felt.floor_div(big_len);
            }
        } else {
            decoded.push(
                BASIC_ALPHABET
                    .chars()
                    .nth(code.to_big_decimal(0).to_usize().unwrap())
                    .unwrap(),
            );
        }
        let (decoded_str, k) = extract_stars(decoded.as_str());
        let mut decoded = String::from(decoded_str);
        if k != 0 {
            let star = last_big.to_string();
            if k % 2 == 0 {
                decoded.push_str(&str::repeat(&star, k / 2 - 1));
                decoded.push(BIG_ALPHABET.chars().next().unwrap());
                let mut basic_iter = BASIC_ALPHABET.chars();
                basic_iter.next();
                decoded.push(basic_iter.next().unwrap());
            } else {
                decoded.push_str(&str::repeat(&star, k / 2 + 1));
            }
        }
    }
    decoded
}

fn extract_stars(mut domain: &str) -> (&str, usize) {
    let mut k = 0;
    let last_char = BIG_ALPHABET.chars().last().unwrap();
    while domain.ends_with(last_char) {
        let mut chars = domain.chars();
        chars.next_back();
        domain = chars.as_str();
        k += 1;
    }
    (domain, k)
}

fn get_coincap_api_key() -> &'static str {
    dotenv!("COINCAP_API_KEY")
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
            threshold: FieldElement::from(10_u128.pow(18) * 50), // 50 eth
            logo: "♦".to_string(),
            rate_api_id: "ethereum".to_string(),
        };
        fetch_events(eth).await.unwrap();
    }

    #[tokio::test]
    async fn test_starknet_id() {
        // stark
        super::address_to_domain(
            FieldElement::from_hex_be(
                "0x1f4055a52c859593e79988bfe998b536066805fe757522ece47945f46f6b6e7",
            )
            .unwrap(),
            FieldElement::from_hex_be(
                "0x6ac597f8116f886fa1c97a23fa4e08299975ecaf6b598873ca6792b9bbfb678",
            )
            .unwrap(),
        )
        .await;

        // eli
        super::address_to_domain(
            FieldElement::from_hex_be(
                "0x48f24d0d0618fa31813db91a45d8be6c50749e5e19ec699092ce29abe809294",
            )
            .unwrap(),
            FieldElement::from_hex_be(
                "0x6ac597f8116f886fa1c97a23fa4e08299975ecaf6b598873ca6792b9bbfb678",
            )
            .unwrap(),
        )
        .await;

        // scott
        super::address_to_domain(
            FieldElement::from_hex_be(
                "0x225bd17f4b4ede26c77673d8d40ec9805ec139a8167cae8d621bd295b260d13",
            )
            .unwrap(),
            FieldElement::from_hex_be(
                "0x6ac597f8116f886fa1c97a23fa4e08299975ecaf6b598873ca6792b9bbfb678",
            )
            .unwrap(),
        )
        .await;

        super::address_to_domain(
            FieldElement::from_hex_be("0x225bd17f4b4ede26c77673d8d3").unwrap(),
            FieldElement::from_hex_be(
                "0x6ac597f8116f886fa1c97a23fa4e08299975ecaf6b598873ca6792b9bbfb678",
            )
            .unwrap(),
        )
        .await;
    }
}
