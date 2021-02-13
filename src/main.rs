use std::vec;

use rustcointlib::*;

mod block_chain;
use block_chain::Blockchain;

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

    let mut blockchain = Blockchain::new();
    let genesis_block = Block::new(vec![Transaction {
        amount:2000.0,
        sender: String::from("Ryan"),
        receiver: String::from("Dan"),
    }]);
    let first_block = Block::new(vec![Transaction {
        amount:1000.0,
        sender: String::from("Paolo"),
        receiver: String::from("Jorge"),
    }]);
    let second_block = Block::new(vec![Transaction {
        amount:1000.0,
        sender: String::from("Thayna"),
        receiver: String::from("Paolo"),
    }]);

    blockchain.add_block(genesis_block);
    blockchain.add_block(first_block);
    blockchain.add_block(second_block);

    println!("{:#?}", blockchain);
    
    let valid = blockchain.is_valid_chain();

    println!("Valid: {}", valid);
}
