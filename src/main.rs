use dotenv::dotenv;
use std::error::Error;
use twitter::tweet;

mod api;
mod db;
mod twitter;

const COINCAP_API_KEY: &str = "COINCAP_API_KEY";
const NODE_PROVIDER_API_KEY: &str = "NODE_PROVIDER_API_KEY";
const TWITTER_OAUTH2_CLIENT_ID: &str = "TWITTER_OAUTH2_CLIENT_ID";
const TWITTER_OAUTH2_CLIENT_SECRET: &str = "TWITTER_OAUTH2_CLIENT_SECRET";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    check_valid_env();

    tweet("Someteaeazzhing".to_string()).await;
    Ok(())
}

fn check_valid_env() {
    dotenv().ok();
    std::env::var(COINCAP_API_KEY).expect("COINCAP_API_KEY must be set.");
    std::env::var(NODE_PROVIDER_API_KEY).expect("NODE_PROVIDER_API_KEY must be set.");
    // TODO These 2 are only required when doing the login, so they prob can be taken out?
    // Maybe this can even turn into another crate?
    std::env::var(TWITTER_OAUTH2_CLIENT_ID).expect("TWITTER_OAUTH2_CLIENT_ID must be set.");
    std::env::var(TWITTER_OAUTH2_CLIENT_SECRET).expect("TWITTER_OAUTH2_CLIENT_SECRET must be set.");
}
/*
mod rpc;
mod starknet;

fn main() -> sc_cli::Result<()> {
    command::run()
}
 */
