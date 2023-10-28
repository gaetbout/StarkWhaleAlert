use log::info;
use std::ops::Deref;
use tokio::sync::Mutex;
use twitter_v2::{
    authorization::{Oauth2Client, Oauth2Token},
    TwitterApi,
};

// TODO Update gitignore when all js gone
const PATH_TO_TOKEN_FILE: &str = "./db/token.json";
pub async fn tweet(text_to_tweet: String) {
    let oauth2_client: Oauth2Client = Oauth2Client::new(
        dotenv!("TWITTER_OAUTH2_CLIENT_ID"),
        dotenv!("TWITTER_OAUTH2_CLIENT_SECRET"),
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
        info!("Refreshing token");
        serde_json::to_writer(
            std::fs::File::create(PATH_TO_TOKEN_FILE).expect("token file not found"),
            token.deref(),
        )
        .expect("couldn't save token");
    }

    info!("Tweeting \n{}", text_to_tweet);
    let res = TwitterApi::new(token.clone())
        .post_tweet()
        .text(text_to_tweet)
        .send()
        .await
        .unwrap()
        .clone();
    let data = res.data().unwrap();
    info!("Tweet id {}", data.id);
}

#[cfg(test)]
mod tests {}
