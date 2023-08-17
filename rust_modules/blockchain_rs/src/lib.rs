use pyo3;
use chrono::prelude::*;
use blake2::{Blake2b512, Blake2s256, Digest};
use bincode;
use serde::{Deserialize, Serialize};

pub fn time_getter() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Serialize, Deserialize)]
struct HashWrapper([u8; 64]);

#[derive(Serialize, Deserialize)]
pub struct Block {
    index: u64,
    timestamp: String,
    proof: u64,
    previous_hash: HashWrapper,

}

impl Block {
    fn generator(&self, index: u64, proof: u64, previous_hash: Blake2b512) -> Block {
        let timestamp = time_getter();
        Block {
            index: index + 1,
            timestamp,
            proof,
            previous_hash
        }
    }

    fn hasher(&self) -> Blake2b512 {
        let serialization = bincode::serialize(&self).unwrap();
        let mut hash = Blake2b512::new();
        hash.update(&serialization);
        hash.finalize();

        hash
        
    }
}

pub struct BlockChain {
    chain: Vec<Block>,
    length: u64,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
