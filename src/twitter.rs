use dotenv::dotenv;
use std::ops::Deref;
use tokio::sync::Mutex;
use twitter_v2::{
    authorization::{Oauth2Client, Oauth2Token},
    TwitterApi,
};

// TODO Update gitignore when all js gone 
const PATH_TO_TOKEN_FILE : &str = "./db/token.json";
use crate::{TWITTER_OAUTH2_CLIENT_ID, TWITTER_OAUTH2_CLIENT_SECRET};
pub async fn tweet(text_to_tweet: String) {
    dotenv().ok();

    let client_id =
        std::env::var(TWITTER_OAUTH2_CLIENT_ID).expect("TWITTER_OAUTH2_CLIENT_ID must be set");
    let client_secret = std::env::var(TWITTER_OAUTH2_CLIENT_SECRET)
        .expect("TWITTER_OAUTH2_CLIENT_SECRET must be set");

    let oauth2_client: Oauth2Client = Oauth2Client::new(
        client_id,
        client_secret,
        "http://127.0.0.1:3000/callback".parse().unwrap(),
    );

    let token: Mutex<Oauth2Token> = Mutex::new(
        serde_json::from_reader(
            std::fs::File::open(PATH_TO_TOKEN_FILE).expect("token file not found"),
        )
        .expect("token file not valid json"),
    );

    let mut token = token.lock().await;
    if oauth2_client
        .refresh_token_if_expired(&mut token)
        .await
        .unwrap()
    {
        println!("Refreshing token");
        serde_json::to_writer(
            std::fs::File::create(PATH_TO_TOKEN_FILE).expect("token file not found"),
            token.deref(),
        )
        .expect("couldn't save token");
    }
    let token = token.clone();

    TwitterApi::new(token)
        .post_tweet()
        .text(text_to_tweet)
        .send()
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {}
