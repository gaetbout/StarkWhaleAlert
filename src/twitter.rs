use log::{error, info};
use std::fs::File;
use twitter_v2::{
    authorization::{Oauth2Client, Oauth2Token},
    TwitterApi,
};

// TODO Update gitignore when all js gone
const PATH_TO_TOKEN_FILE: &str = "./db/token.json";
pub async fn tweet(text_to_tweet: String) {
    info!("Tweet start");

    let oauth2_client: Oauth2Client = Oauth2Client::new(
        dotenv!("TWITTER_OAUTH2_CLIENT_ID"),
        dotenv!("TWITTER_OAUTH2_CLIENT_SECRET"),
        "http://127.0.0.1:3000/callback".parse().unwrap(),
    );

    let mut token: Oauth2Token =
        serde_json::from_reader(File::open(PATH_TO_TOKEN_FILE).expect("Cound not open file"))
            .expect("token file not valid json");

    if oauth2_client
        .refresh_token_if_expired(&mut token)
        .await
        .unwrap()
    {
        info!("Refreshing token");
        serde_json::to_writer(
            std::fs::File::create(PATH_TO_TOKEN_FILE).expect("token file not found"),
            &token,
        )
        .expect("couldn't save token");
    }

    info!("Tweeting \n{}", text_to_tweet);
    let res = TwitterApi::new(token)
        .post_tweet()
        .text(text_to_tweet)
        .send()
        .await;

    if res.is_err() {
        error!("Twitter error\n{:?}", res.unwrap_err());
    }
}

#[cfg(test)]
mod tests {

    use super::tweet;
    use crate::logger;

    #[tokio::test]
    async fn test_tweet() {
        logger::init();
        const AVOID_DUP: u8 = 6;
        tweet(format!("First tweet {}", AVOID_DUP)).await;
        tweet(format!("Second tweet {}", AVOID_DUP)).await;
    }
}
