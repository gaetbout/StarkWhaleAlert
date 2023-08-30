use dotenv::dotenv;
use std::error::Error;

mod api;

const COINCAP_API_KEY: &str = "COINCAP_API_KEY";
const NODE_PROVIDER_API_KEY: &str = "NODE_PROVIDER_API_KEY";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    check_valid_env();

    Ok(())
}

fn check_valid_env() {
    dotenv().ok();
    std::env::var(COINCAP_API_KEY).expect("COINCAP_API_KEY must be set.");
    std::env::var(NODE_PROVIDER_API_KEY).expect("NODE_PROVIDER_API_KEY must be set.");
}
/*
mod rpc;
mod starknet;

fn main() -> sc_cli::Result<()> {
    command::run()
}
 */
