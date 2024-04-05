use serde::{Serialize, Deserialize};
use casper_types::{PublicKey, Signature};
#[derive(Serialize, Deserialize)]
pub struct SignatureInput{
    pub data: Vec<u8>,
    pub signature: Signature,
    pub public_key: PublicKey
}

#[derive(Serialize, Deserialize)]
pub struct SignatureBatchInput{
    pub signatures: Vec<SignatureInput>
}