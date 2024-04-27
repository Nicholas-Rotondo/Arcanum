use chrono::Utc;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct ArcanumBlock {
    timestamp: u64,
    hash: String,
    previous_hash: String,
    nonce: u64,
    id: i64,
}

struct ArcanumChain {
    chain: vec![ArcanumBlock]
}

impl ArcanumBlock {

    // See standard on intializing
    fn new(timestamp: i64, hash: String, previous_hash: String, nonce: u64, id: i64) -> ArcanumBlock {
        let block = ArcanumBlock {
            timestamp: UTC::now(),
            hash: create_hash(),
            previous_hash,
            nonce,
            id,
        };
    }

    fn create_hash() -> String {
        let mut hasher = Sha256::new();
        let data = b"Hello world!";
        hasher.update(data);
         // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
        hasher.update("String data");
        // Note that calling `finalize()` consumes hasher
        let hash = hasher.finalize();
        println!("Binary hash: {:?}", hash)
    }
}