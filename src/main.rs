use api::Token;
use dotenv::dotenv;
use std::error::Error;
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
    check_valid_env();
    // twitter::tweet("Someteaeazzhing".to_string()).await;

    Ok(())
}

fn check_valid_env() {
    dotenv().ok();
    std::env::var("COINCAP_API_KEY").expect("COINCAP_API_KEY must be set.");
    std::env::var("NODE_PROVIDER_API_KEY").expect("NODE_PROVIDER_API_KEY must be set.");
    // TODO These 2 are only required when doing the login, so they prob can be taken out?
    // Maybe this can even turn into another crate?
    std::env::var("TWITTER_OAUTH2_CLIENT_ID").expect("TWITTER_OAUTH2_CLIENT_ID must be set.");
    std::env::var("TWITTER_OAUTH2_CLIENT_SECRET")
        .expect("TWITTER_OAUTH2_CLIENT_SECRET must be set.");
}
