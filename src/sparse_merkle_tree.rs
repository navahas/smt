use crate::utils::*;
use sha3::{Digest, Keccak256};

const MAX_LEVELS: usize = 8;
const MAX_LEAVES: usize = 1 << MAX_LEVELS; // bitwise for 2^x

#[derive(Debug)]
pub struct SparseMerkleTree {
    leaves: Vec<Vec<[u8; 32]>>,
    // precomputed hashes per level
    zero: Vec<[u8; 32]>,
    root: Option<[u8; 32]>,
    max_leaves: usize,
    max_levels: usize,
}

impl SparseMerkleTree {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
            zero: Vec::new(),
            root: None,
            max_leaves: MAX_LEAVES,
            max_levels: MAX_LEVELS,
        }
    }

    pub fn init(&self) -> Self {
        let zero_leaves = precompute_zero_hashes(self.max_levels);
        let root = zero_leaves[self.max_levels];
        Self {
            leaves: Vec::new(),
            zero: zero_leaves,
            root: Some(root),
            max_levels: MAX_LEVELS,
            max_leaves: MAX_LEVELS
        }
    }

}
