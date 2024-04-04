#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

extern crate alloc;
use alloc::{vec, vec::Vec};

use risc0_compat::compat::env::{commit, read};
use risc0_compat::compat::crypto::sha::hash;
risc0_compat::entry!(main);

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = read();
    // TODO: do something with the input
    let some_compat_hash = hash(&vec![0u8;32]);
    // write public output to the journal
    commit(&input);
}
