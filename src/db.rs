use std::default::Default;

use koit::{format::Json, FileDatabase};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
struct Data {
    last_processsed_block: u128,
}

async fn get_db(path: &str) -> FileDatabase<Data, Json> {
    FileDatabase::<Data, Json>::load_from_path_or_default(path)
        .await
        .unwrap()
}

pub async fn get_last_processsed_block(path: &str) -> u128 {
    get_db(path)
        .await
        .read(|data| data.to_owned())
        .await
        .last_processsed_block
}

pub async fn set_last_processsed_block(path: &str, last_processsed_block: u128) {
    let db = get_db(path).await;
    db.write(|data| {
        data.last_processsed_block = last_processsed_block;
    })
    .await;
    db.save().await.unwrap();
}
#[cfg(test)]
mod tests {

    use super::{get_last_processsed_block, set_last_processsed_block};
    use std::fs;

    #[tokio::test]
    async fn test_db() {
        let path = "./test_db";
        set_last_processsed_block(path, 12).await;
        let number = get_last_processsed_block(path).await;
        println!("number {:?}:", number);
        assert!(fs::metadata(path).is_ok(), "File should exist");
        fs::remove_file(path).unwrap();
        assert!(fs::metadata(path).is_err(), "File shouldn't exist anymore");
    }
}
