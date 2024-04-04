#[cfg(not(feature="risc0"))]
pub mod sha{
    extern crate alloc;
    use alloc::vec::Vec;
    use sha2::{Sha256, Digest};
    pub fn hash(data: &[u8]) -> Vec<u8>{
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

#[cfg(feature="risc0")]
pub mod sha{
    extern crate alloc;
    use alloc::{borrow::ToOwned, vec::Vec};
    use serde::Serialize;
    use risc0_zkvm::sha::{Digest, Impl, Sha256};
    pub fn hash(data: &[u8]) -> Vec<u8>{
        Impl::hash_bytes(data).as_bytes().to_vec()
    }
}