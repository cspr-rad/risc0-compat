use risc0_compat::compat::{env::{commit, read}, crypto::sha};
use serde::{Serialize, Deserialize};
extern crate alloc;
use alloc::{vec, vec::Vec};
#[derive(Serialize, Deserialize)]
struct Output{
    sha256_hashes: Vec<u8>
}

fn main() {
    let some_data: Vec<u8> = vec![0,0,0];
    let x = sha::hash(&some_data);
    println!("Length: {:?}", x.len());
    println!("Hash: {:?}", &x);
}