///use pyo3;
use chrono::prelude::*;
use blake2::{Blake2b512, Digest, digest::{KeyInit, Update}};
use bincode;
use serde::{Deserialize, Serialize};
///use serde_json;

pub fn time_getter() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    index: usize,
    timestamp: String,
    proof: u64,
    previous_hash: Vec<u8>,

}

impl Block {
    pub fn generator(index: usize, previous_hash: Vec<u8>) -> Block {
        let timestamp = time_getter();
        let proof = if index == -1 {0} else {self.proof_of_work(&self)};

        Block {
            index: index + 1,
            timestamp,
            proof,
            previous_hash
        }
    }

    pub fn hasher(&self) -> Vec<u8> {
        let serialization = bincode::serialize(&self).unwrap();
        let mut hash = Blake2b512::new();
        Update::update(&mut hash, &serialization);
        let result = hash.finalize();

        result.to_vec()
        
    }
}

pub struct BlockChain {
    chain: Vec<Block>,
    index: usize,
}

impl BlockChain {
    fn generator() -> BlockChain {
        BlockChain {
            chain: vec![Block::generator(-1, vec![0])],
            index: 0
        }
    }

    fn cur_block(&self) -> &Block {
        &self.chain[self.index]
    }

    fn proof_of_work(&self) -> u64 {
        let current_proof = self.cur_block().proof;

        let mut new_proof = 1;
        
        loop {
            let hash_operation = format!("{:x}",
                Blake2b512::new()
                    .chain(format!("{:x}", new_proof * new_proof - current_proof * current_proof))
                    .finalize());

            if hash_operation.starts_with("000") {
                break
            }

            new_proof += 1;
            }
        new_proof
    }

    pub fn blockchain_extender(&self) -> () {
        self.chain.push(Block::generator(self.index, Block::hasher(self.cur_block()));
    };
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
