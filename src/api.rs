
use reqwest::header;
use reqwest::header::HeaderValue;
use std::time::Duration;
pub async fn fetch_coin(coin_id: &str) {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_static("Bearer ${coincapApiKey}"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed building the client");

    let get_link: String = format!("{}{}", "https://api.coincap.io/v2/assets/", coin_id);

    let ethereum = client
        .get(get_link)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:}", ethereum);
}

#[cfg(test)]
mod tests {
    use super::fetch_coin;
    #[tokio::test]
    async fn it_works() {
        fetch_coin("ethereum").await;
    }
}
