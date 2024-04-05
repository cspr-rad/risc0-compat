use methods::{
    ECDSA_ELF, ECDSA_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use signature_types::{SignatureInput, SignatureBatchInput};
use kairos_crypto::implementations::Signer;
use kairos_crypto::CryptoSigner;
use dotenvy::dotenv;
use std::path::PathBuf;
use std::env;
use casper_types::crypto::Signature;
use casper_types::bytesrepr::{ToBytes, FromBytes};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    
    /*
        Sign a bunch of messages to generate a set of signatures that are to be verified
        by the guest program
    */
    dotenv().ok();
    let path_to_secret_key = env::var("PATH_TO_SECRET_KEY_PEM")
        .expect("Missing environment variable PATH_TO_SECRET_KEY_PEM");
    let signer = Signer::from_private_key_file(PathBuf::from(path_to_secret_key)).expect("Failed to construct signer!");
    let mut signature_batch_input: SignatureBatchInput = SignatureBatchInput{
        signatures: Vec::new()
    };
    let raw_message: Vec<u8> = vec![0;32];
    for i in 0..1{
        let signature_bytes: Vec<u8> = signer.sign(&raw_message).expect("Failed to sign!");
        let signature: Signature = Signature::from_bytes(signature_bytes.as_ref()).expect("Failed to construct Signature!").0;
        let signature_input: SignatureInput = SignatureInput{
            data: raw_message.clone(),
            public_key: signer.public_key.clone(),
            signature: signature
        };
        signature_batch_input.signatures.push(signature_input);
    }
    /*
        Commit the signatures to the guest program env
    */
    let env = ExecutorEnv::builder()
        .write(&signature_batch_input)
        .unwrap()
        .build()
        .unwrap();

    /*
        Generate and verify the proof for the guest program execution
    */
    let prover = default_prover();
    let receipt = prover
        .prove(env, ECDSA_ELF)
        .unwrap();
    receipt
        .verify(ECDSA_ID)
        .unwrap();
}
