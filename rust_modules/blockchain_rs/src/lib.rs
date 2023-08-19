///use pyo3;
use chrono::prelude::*;
use blake2::{Blake2b512, Digest, digest::Update};
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
    pub fn generator(index: usize, proof: u64, previous_hash: Vec<u8>) -> Block {
        let timestamp = time_getter();

        Block {
            index,
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
            chain: vec![Block::generator(0, 0, vec![0])],
            index: 0
        }
    }

    fn cur_block(&self) -> &Block {
        &self.chain[self.index]
    }

    // Doesn't work. As it would seem, this still is the hardest part for me.
    // I need to find a place to study such algorithms, since as of now I can't come up with one.
    // Or invest my time in implementing FBA, since that was my idea while I was working on the Blockchain before the summer.
    // I should perhaps try to import the node code that I have in Python into Rust.
    fn proof_of_work(&self, difficulty: &str) -> u64 {
        let current_proof = self.cur_block().proof;

        let mut new_proof = current_proof + 1;
        
        // loop {
        //     let hash_operation = format!(
        //         "{:x}",
        //         Blake2b512::new()
        //             .chain(format!("{:x}", new_proof * new_proof + current_proof * current_proof))
        //             .finalize()
        //         );

        //     if hash_operation.starts_with(difficulty) {
        //         break;
        //     }

        //     new_proof += 1;
        //     println!("{new_proof}");
        //     }
        new_proof
    }

    pub fn blockchain_extender(&mut self) {
        let proof = self.proof_of_work("000");
        let previous_hash = self.cur_block().hasher();
        let new_block = Block::generator(self.index + 1, proof, previous_hash);
        self.chain.push(new_block);
        self.index += 1;
        }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_blockchain() -> BlockChain {
        BlockChain::generator()
    }

    #[test]
    fn blockchain_generation_test() {
        let blockchain = setup_blockchain();
        assert_eq!(blockchain.index, 0);
        assert_eq!(blockchain.chain.len(), 1);
    }

    #[test]
    fn genesis_block_correctness_test() {
        let blochchain = setup_blockchain();
        let genesis_block = blochchain.cur_block();
        assert_eq!(genesis_block.index, 0);
        assert_eq!(genesis_block.proof, 0);
        assert_eq!(genesis_block.previous_hash, vec![0])
    }

    #[test]
    fn blockchain_extender_test() {
        let mut blockchain = setup_blockchain();
        blockchain.blockchain_extender();
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.index, 1);
    }

    #[test]
    fn new_block_correctness_test() {
        let mut blockchain = setup_blockchain();
        blockchain.blockchain_extender();
        let block = blockchain.cur_block();
        assert_eq!(block.index, 1);
        assert_ne!(block.proof, 0);
        assert_ne!(block.previous_hash, vec![0]);
    }

    // #[test]
    // fn proof_of_work_test() {
    //     let target_difficulty = "000";
    //     let mut blockchain = setup_blockchain();
    //     blockchain.blockchain_extender();
    //     let proof = blockchain.proof_of_work(target_difficulty);
    //     assert_eq!(&format!("{:x}", proof)[..3], target_difficulty)
    // }
}
