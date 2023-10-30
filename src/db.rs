use std::default::Default;

use koit::{format::Json, FileDatabase};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
struct Data {
    last_processed_block: u64,
}

const LAST_BLOCK_FILE_PATH: &str = "./db/block.json";

// TODO Create default DB if not existing.

async fn get_db() -> FileDatabase<Data, Json> {
    FileDatabase::<Data, Json>::load_from_path_or_default(LAST_BLOCK_FILE_PATH)
        .await
        .expect("Error: get_db")
}

pub async fn get_last_processed_block() -> u64 {
    get_db()
        .await
        .read(|data| data.to_owned())
        .await
        .last_processed_block
}

pub async fn set_last_processed_block(last_processed_block: u64) {
    info!("Written block {}", last_processed_block);
    let db = get_db().await;
    db.write(|data| {
        data.last_processed_block = last_processed_block;
    })
    .await;
    db.save().await.unwrap();
}
#[cfg(test)]
mod tests {

    use super::{get_last_processed_block, set_last_processed_block};
    use std::fs;

    #[tokio::test]
    async fn test_db() {
        let path = "./test_db";
        set_last_processed_block(12).await;
        let number = get_last_processed_block().await;
        println!("number {:?}:", number);
        assert!(fs::metadata(path).is_ok(), "File should exist");
        fs::remove_file(path).unwrap();
        assert!(fs::metadata(path).is_err(), "File shouldn't exist anymore");
    }
}
