use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ArcanumBlock {
    timestamp: DateTime<Utc>,
    hash: String,
    previous_hash: String,
    nonce: u64,
    id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct JSONHash {
    timestamp: DateTime<Utc>,
    previous_hash: String,
    nonce: u64,
    id: i64,
}

struct ArcanumChain {
    chain: Vec<ArcanumBlock>
}

impl ArcanumBlock {
    pub fn new(hash: String, previous_hash: String, nonce: u64, id: i64) -> ArcanumBlock {
        let now = Utc::now();

        let hash = Self::create_hash(&now, &previous_hash, nonce, id).unwrap();
        let block = ArcanumBlock {
            timestamp: now,
            hash,
            previous_hash,
            nonce,
            id,
        };

        return block;
    }

    fn create_hash(timestamp: &DateTime<Utc>, previous_hash: &String, nonce: u64, id: i64) -> Result<String, serde_json::Error> {
        let mut hasher = Sha256::new();
        let json_hash = JSONHash {
            timestamp: *timestamp,
            previous_hash: previous_hash.clone(),
            nonce,
            id
        };
        let serialized = serde_json::to_string(&json_hash)?;
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }
}