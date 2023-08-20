use dotenv::dotenv;
use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let mailcoach_api_token =
        std::env::var("COINCAP_API_KEY").expect("COINCAP_API_KEY must be set.");
    println!("Somethng {mailcoach_api_token}");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_staic("Bearer ${coincapApiKey}"),
    );

    let client = reqwest::Client::builder().default_headers(headers).build()?;

    let get_link: String = format!("{}{}", "https://api.coincap.io/v2/assets/", "ethereum");

    let doge = client
        .get(get_link)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    println!("{:}", doge);
    Ok(())
}
