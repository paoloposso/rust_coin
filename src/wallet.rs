extern crate rand;
extern crate ed25519_dalek;

use ed25519_dalek::{PublicKey,SecretKey,Keypair};
use rand::rngs::{OsRng};

#[derive(Debug)]
pub struct Wallet {
    pub public: PublicKey,
    pub secret: SecretKey
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Wallet {
            public: keypair.public,
            secret: keypair.secret,
        }
    }
}