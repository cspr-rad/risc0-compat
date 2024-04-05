#![no_main]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

use risc0_compat::compat::env::{commit, read};
use risc0_compat::compat::crypto::sha::hash;
risc0_compat::entry!(main);

fn main() {
    let input: u32 = read();
    let some_compat_hash = hash(&vec![0u8;32]);
    commit(&input);
}
