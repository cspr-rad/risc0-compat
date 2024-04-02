use serde::Serialize;
pub use super::Risc0 as EnvImpl;
/// Our compat for https://docs.rs/risc0-zkvm-guest/latest/risc0_zkvm_guest/env/index.html
/// https://docs.rs/risc0-zkvm/latest/risc0_zkvm/guest/env/index.html
pub trait Env {
    fn commit<T: Serialize>(data: &T);
}

#[cfg(not(feature = "risc0"))]
impl Env for EnvImpl {
    fn commit<T: Serialize>(data: &T) {
        // mock implementation
    }
}

#[cfg(feature = "risc0")]
mod risc0_impl {
    use super::Env;
    pub use super::EnvImpl;
    use serde::{Serialize, Deserialize};
    use risc0_zkvm::guest::env;
    impl Env for EnvImpl {
        fn commit<T: Serialize>(data: &T)
        {
            env::commit(data);
        }
    }
}

#[cfg(feature = "risc0")]
pub use risc0_impl::*;