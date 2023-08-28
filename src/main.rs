use dotenv::dotenv;
use std::error::Error;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    check_valid_env();
    
    api::lama().await;

    Ok(())
}

fn check_valid_env(){
    dotenv().ok();
    let mailcoach_api_token =
        std::env::var("COINCAP_API_KEY").expect("COINCAP_API_KEY must be set.");
        println!("Somethng {mailcoach_api_token}");
}