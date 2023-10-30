use log::info;
use std::fs::File;
use twitter_v2::{authorization::Oauth2Token, TwitterApi};

// TODO Update gitignore when all js gone
const PATH_TO_TOKEN_FILE: &str = "./db/token.json";
pub async fn tweet(text_to_tweet: String) {
    info!("Tweet start");

    let token: Oauth2Token =
        serde_json::from_reader(File::open(PATH_TO_TOKEN_FILE).expect("Cound not open file"))
            .expect("token file not valid json");

    info!("Tweeting \n{}", text_to_tweet);
    let res = TwitterApi::new(token)
        .post_tweet()
        .text(text_to_tweet)
        .send()
        .await;

    match res {
        Ok(val) => {
            info!("Refreshing token");
            serde_json::to_writer(
                std::fs::File::create(PATH_TO_TOKEN_FILE).expect("token file not found"),
                val.client().auth(),
            )
            .expect("couldn't save token");
        }
        Err(err) => {
            info!("ERROR: \n{:?}", err)
        }
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
