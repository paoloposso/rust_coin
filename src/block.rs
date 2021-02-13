use super::*;
use transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {  
        let time = now();
        let empty_string = String::new();  
        let nonce = 0u64;
        Block {
            hash: empty_string.clone(),
            timestamp: time,
            pre_hash: empty_string.clone(),
            transactions,
            nonce
        }
    }

    pub fn set_pre_hash(&mut self, pre_hash: String) {
        self.pre_hash = pre_hash;
    }

    pub fn set_hash(&mut self) {
        self.hash = calculate_hash(&self.pre_hash, &self.transactions, &self.timestamp, &self.nonce);
    }

    pub fn mine(&mut self) {
        let target = get_difficult_string();
        while &self.hash[..DIFFICULT_LEVEL as usize] != target {
            self.nonce += 1;
            self.hash = calculate_hash(&self.pre_hash, &self.transactions, &self.timestamp, &self.nonce);
        }
        println!("Block mined!");
    }

    pub fn has_valid_transactions(&self) -> bool {
        for tran in &self.transactions {
            if !tran.is_valid_trasaction() {
                return false;
            }
        }
        true
    }
}