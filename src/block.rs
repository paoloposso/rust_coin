use super::*;
use transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>
}

impl Block {
    pub fn new(pre_hash: String, transaction: Vec<Transaction>) -> Self {  
        let time = now();      
        Block {
            hash: calculate_hash(&pre_hash, &transaction, time).to_owned(),
            timestamp: time,
            pre_hash,
            transaction
        }
    }
}