use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        // TODO: create and return a new initial block
        let mut prev_hash = GenericArray::default();
        for i in &mut prev_hash {
            *i = 0u8;
        }

        Block {
            prev_hash: prev_hash,
            generation: 0,
            difficulty: difficulty,
            data: "".to_string(),
            proof: None,
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        // TODO: create and return a block that could follow `previous` in the chain
        Block {
            prev_hash: previous.hash(),
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data: data,
            proof: previous.proof,
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        // TODO: return the hash string this block would have if we set the proof to `proof`.
        // Brought from the Hex conversion given in the assignmnet
        let prev_hash = self.prev_hash;
        let mut output = String::new();
        write!(&mut output, "{:02x}", prev_hash).unwrap();
        let result = format!("{}:{}:{}:{}:{}", output, self.generation, self.difficulty, self.data, proof);
        return result;
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // TODO: return the block's hash as it would be if we set the proof to `proof`.
        let hash_proof = Block {
            prev_hash: self.prev_hash,
            generation: self.generation,
            difficulty: self.difficulty,
            data: self.data.to_string(),
            proof: Some(proof),
        };

        let str_format = self.hash_string_for_proof(proof);
        //crate Sha256 to hash the above strings
        let mut hasher = Sha256::new();
        hasher.update(str_format);
        let hash = hasher.finalize();
        return hash;
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        // TODO: would this block be valid if we set the proof to `proof`?
        let n_bytes = self.difficulty/8;
        let n_bits = self.difficulty%8;
        let hash = self.hash_for_proof(proof);
        let last_n_bytes = 32 - n_bytes as usize;

        //Check each of the last n_bytes bytes are 0u8.
        for i in last_n_bytes..32 {
            if hash[i as usize] != 0u8 {
                return false;
            }
        }
        //Check that the next byte (from the end) is divisible by 1<<n_bits
        if hash[last_n_bytes-1 as usize]%(1<<n_bits) != 0 {
            return false;
        }

        return true;
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // TODO: with `workers` threads, check proof values in the given range, breaking up
        // into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.
        let mut work_queue = WorkQueue::new(workers); //Create WorkQueue
        //Set the range and size using the parameter chucks, start, end
        let mut range = chunks; //range: How many chunks
        let mut size = 1; //size: 'chunks' tasks in a queue
        if end - start < chunks {
            range = end - start;
        } else {
            size = (end - start) / chunks;
        }

        //Use a queue::WorkQeueue and sync::Arc to wrap a clone of self for sharing
        for i in 0..range-1 {
            let new_task = MiningTask::new(sync::Arc::new(self.clone()), i*size, (i+1)*size-1);
            work_queue.enqueue(new_task).unwrap();
        }

        //check proof-of-work values until you find for which block.is_valid_for_proof() returns true.
        //Return the first valid proof found.
        loop {
            let rcv = work_queue.recv();
            if self.is_valid_for_proof(rcv) {
                work_queue.shutdown();
                return rcv;
            }
        }
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    // TODO: more fields as needed
    start: u64,
    end: u64,
}

impl MiningTask {
    // TODO: implement MiningTask::new(???) -> MiningTask
    pub fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask {
            block: block,
            start: start,
            end: end,
        }
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        // TODO: what does it mean to .run?
        for i in self.start..=self.end {
            if self.block.is_valid_for_proof(i as u64) {
                return Some(i as u64);
            }
        }

        return None;
    }
}
