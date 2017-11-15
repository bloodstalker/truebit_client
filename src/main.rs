extern crate web3;
extern crate rustc_hex;

use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};
use rustc_hex::FromHex;

fn main() {
  let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
  let web3 = web3::Web3::new(transport);
  let accounts = web3.eth().accounts().wait().unwrap();

    //Get current balance

  let balance = web3.eth().balance(accounts[0], None).wait().unwrap();

  println!("Balance: {}", balance);

  // Get the contract bytecode for instance from Solidity compiler
  let bytecode: Vec<u8> = include_str!("../build/SimpleStorage.bin").from_hex().unwrap();
  // Deploying a contract
  let contract = Contract::deploy(web3.eth(), include_bytes!("../build/SimpleStorage.abi")).unwrap()
    .execute(bytecode, (), accounts[0])
    .unwrap()
  	.wait()
  	.unwrap();

  println!("{}", contract.address());
}