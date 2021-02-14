# About
Blockchain implementation in Rust using the examples of book Blockchain For Rust Developers by Ayush Kumar Mishra

# Structure

## block_chain
Contains references to block file and the main methods that allow the blockchain to work.

## block

## transaction

## wallet

## lib
Contains common methods and functions, such as hashing generation and date.

## dependencies on toml file
rand version "0.7.0" is used because ed25519-dalek depends on it. Otherwise the Keypair generation won't work because of changes ocurred on later versions.