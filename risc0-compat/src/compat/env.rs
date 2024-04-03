use serde::{Serialize, de::DeserializeOwned};

pub fn commit<T: Serialize>(data: &T){
    #[cfg(not(feature="risc0"))]
    {
        // mock is to be implemented
    }
    #[cfg(feature="risc0")]
    {
        use risc0_zkvm::guest::env;
        env::commit(data);
    }
}

pub fn read<T: DeserializeOwned>(){
    #[cfg(not(feature="risc0"))]
    {
        // mock is to be implemented        
    }
    #[cfg(feature="risc0")]
    {
        use risc0_zkvm::guest::env;
        env::read::<T>();
    }
}