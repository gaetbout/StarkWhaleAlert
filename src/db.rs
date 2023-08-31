
use std::default::Default;

use koit::{FileDatabase, format::Json};
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
struct Data {
    last_processsed_block: u128,
    twitter_refresh_token: String,
}

async fn get_db(path:&str) -> FileDatabase<Data,Json> {
    FileDatabase::<Data, Json>::load_from_path_or_default(path).await.unwrap()
}

pub async fn get_last_processsed_block(path:&str) -> u128 {
    get_db(path).await.read(|data| data.to_owned()).await.last_processsed_block
}

pub async fn get_twitter_refresh_token(path:&str) -> String {
    get_db(path).await.read(|data| data.to_owned()).await.twitter_refresh_token
}

pub async fn set_last_processsed_block(path:&str, last_processsed_block: u128) {
    let db = get_db(path).await;
    db.write(|data| {
        data.last_processsed_block = last_processsed_block;
    }).await;
    db.save().await.unwrap();
}

pub async fn set_twitter_refresh_token(path:&str, twitter_refresh_token:String)  {
    let db = get_db(path).await;
    db.write(|data| {
        data.twitter_refresh_token = twitter_refresh_token;
    }).await;
    db.save().await.unwrap();
}
#[cfg(test)]
mod tests {

    use super::{get_last_processsed_block, get_twitter_refresh_token, set_last_processsed_block, set_twitter_refresh_token};

    #[tokio::test]
    async fn test_db() {
        let path = "./test_db";
        set_last_processsed_block(path, 12).await;
        let number = get_last_processsed_block(path).await;
        let twitter = get_twitter_refresh_token(path).await;
        println!("number {:?}:", number);
        println!("twitter {:?}:", twitter);
    }

}
