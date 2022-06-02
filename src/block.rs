use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Block {
    pub index: usize,
    hash: String,
    current_time: DateTime<Utc>,
    pub data: String,
    pub previous_hash: String,
}

fn get_next_hash(index: usize, data: String, previous_hash: String) -> (String, DateTime<Utc>) {
    let current_time = Utc::now();
    let mut hasher = Sha256::new();
    let hash_data = format!("{}{}{}{}", index, previous_hash, current_time, data);

    hasher.update(hash_data);

    let hash = hasher.finalize();
    (base16ct::lower::encode_string(&hash), current_time)
}

impl Block {
    pub fn new(index: usize, data: String, previous_hash: String) -> Block {
        let next_hash = get_next_hash(index, data.clone(), previous_hash.clone());
        Block {
            index,
            data,
            previous_hash,
            hash: next_hash.0,
            current_time: next_hash.1,
        }
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}
