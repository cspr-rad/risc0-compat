use serde::{Serialize, de::DeserializeOwned};

pub fn commit<T: Serialize>(data: &T){
    #[cfg(not(feature="risc0"))]
    {
        /*
            write serialized data to SQLite
        */
    }
    #[cfg(feature="risc0")]
    {
        use risc0_zkvm::guest::env;
        env::commit(data)
    }
}

#[cfg(not(feature="risc0"))]
pub fn read<T: DeserializeOwned>(){ // -> T{
    /*
        return deserialized data from SQLite
    */

}

#[cfg(feature="risc0")]
pub fn read<T: DeserializeOwned>() -> T{
    use risc0_zkvm::guest::env;
    env::read::<T>()
}