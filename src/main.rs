// use std::vec;

use rustcointlib::*;

mod block_chain;
use block_chain::Blockchain;
use ed25519_dalek::Keypair;

mod wallet;

fn main() {
    // let first_block = Block::new("0".to_owned(), vec![Transaction {
    //   sender: String::from("Ryan"),
    //   receiver: String::from("Dan"),
    //   amount: 2000.00
    // }]);

    // let hash1 = first_block.hash;

    // let sec_block = Block::new(hash1.to_owned(), vec![Transaction {
    //     sender: String::from("Ryan"),
    //     receiver: String::from("Dan"),
    //     amount: 2000.00
    // }]);

    // println!("{:#?}", sec_block);
    // println!("{:#?}", first_block);

    // let mut blockchain = Blockchain::new();
    // let genesis_block = Block::new(vec![Transaction {
    //     amount:2000.0,
    //     sender: String::from("Ryan"),
    //     receiver: String::from("Dan"),
    // }]);
    // let first_block = Block::new(vec![Transaction {
    //     amount:1000.0,
    //     sender: String::from("Paolo"),
    //     receiver: String::from("Jorge"),
    // }]);
    // let second_block = Block::new(vec![Transaction {
    //     amount:1000.0,
    //     sender: String::from("Thayna"),
    //     receiver: String::from("Paolo"),
    // }]);

    // let mut blockchain = Blockchain::new();
    // blockchain.add_new_transaction(Transaction {sender: String::from("Paolo"), receiver: String::from("Victor"), amount: 1000.0});
    // blockchain.add_new_transaction(Transaction {sender: String::from("Jorge"), receiver: String::from("Paolo"), amount: 1000.0});
    // blockchain.add_new_transaction(Transaction {sender: String::from("Thayna"), receiver: String::from("Paolo"), amount: 1000.0});

    // blockchain.mine_unmined_transactions("First Miner".to_owned());

    // println!("{:#?}", blockchain);
    // println!("Valid: {}", blockchain.is_valid_chain());


}
