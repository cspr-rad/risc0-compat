#![no_main]
//#![no_std] // see host/Cargo.toml
use risc0_zkvm::guest::env;
use risc0_zkvm::guest::env::commit;
risc0_zkvm::guest::entry!(main);
use casper_types::{crypto, PublicKey, Signature};
use signature_types::{SignatureInput, SignatureBatchInput};

fn main() {
    let signature_batch: SignatureBatchInput = env::read();
    for signature_input in signature_batch.signatures{
        verify(&signature_input.data, &signature_input.signature, &signature_input.public_key);
    }
    commit(&1u32);
}

pub fn verify<T: AsRef<[u8]>>(data: T, signature: &Signature, public_key: &PublicKey){
    crypto::verify(data, &signature, &public_key).expect("Failed to verify!");
}