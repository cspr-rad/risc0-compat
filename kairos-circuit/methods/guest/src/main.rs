#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


//use risc0_zkvm::guest::env;
//risc0_zkvm::guest::entry!(main);
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Output{
    x: u16
}

// import mock
use risc0_compat::compat::{env::{Env, EnvImpl}, Risc0};
risc0_compat::entry!(main);

fn main() {
    // TODO: Implement your guest code here

    // read the input
    // let input: u32 = env::read();
    let output: Output = Output{
        x: 1u16
    };
    // TODO: do something with the input
    Risc0::commit(&output);
    // write public output to the journal
    // env::commit(&input);
}
