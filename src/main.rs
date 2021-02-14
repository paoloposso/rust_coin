// use std::vec;

use rustcointlib::*;

mod block_chain;
use block_chain::Blockchain;
use ed25519_dalek::Keypair;
use wallet::Wallet;

mod wallet;

fn main() {
    let mut blockchain = Blockchain::new();

    let paolo_key = Wallet::new();
    let jorge_key = Wallet::new();
    let miner_key = Wallet::new();

    let mut fist_transaction = Transaction {
        sender: Some(paolo_key.public),
        receiver: Some(jorge_key.public),
        amount: 2000.0,
        signature: None,
    };
    fist_transaction.sign_transaction(Keypair {
        public: paolo_key.public,
        secret: paolo_key.secret
    });
    blockchain.add_new_transaction(fist_transaction);
    blockchain.mine_unmined_transactions(miner_key.public);

    print!("{}", blockchain.is_valid_chain());
    print!("{:#?}", blockchain);
}
