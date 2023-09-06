use dotenv::dotenv;
use std::error::Error;
#[macro_use]
extern crate dotenv_codegen;

mod api;
mod db;
mod logger;
mod twitter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    check_valid_env();
    // tweet("Someteaeazzhing".to_string()).await;
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
