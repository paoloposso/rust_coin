mod block;
pub use block::Block;

pub mod transaction;
use std::time::{SystemTime, UNIX_EPOCH};

pub use transaction::Transaction;

const DIFFICULT_LEVEL: i32 = 2;
pub const MINING_REWARD: f32 = 100f32;

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn calculate_hash(pre_hash: &String, transactions: &Vec<Transaction>, timestamp: &u64, nonce: &u64) -> String {

    let _ = transactions.iter();
    let _ = transactions.iter().flat_map(|transaction| transaction.bytes());
    let _ = transactions.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>();

    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(transactions.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>());
    bytes.extend(pre_hash.as_bytes());
    bytes.extend(&nonce.to_ne_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}

pub fn get_difficult_string() -> String {
    let mut s = String::new();
    for _i in 0..DIFFICULT_LEVEL {
        s.push_str("0");
    }

    s
}