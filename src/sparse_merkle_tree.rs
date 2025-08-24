use crate::utils::*;
use sha3::{Digest, Keccak256};

const MAX_LEVELS: usize = 8;
const MAX_LEAVES: usize = 1 << MAX_LEVELS; // bitwise for 2^x

#[derive(Debug)]
pub struct SparseMerkleTree {
    leaves: Vec<Vec<[u8; 32]>>,
    // precomputed hashes per level
    zero: Vec<[u8; 32]>,
    root: [u8; 32],
    max_leaves: usize,
    max_levels: usize,
}

impl SparseMerkleTree {
    pub fn new() -> Self {
        let zero_leaves = precompute_zero_hashes(MAX_LEVELS);
        let root = zero_leaves[MAX_LEVELS];
        Self {
            leaves: Vec::new(),
            zero: zero_leaves,
            root: root,
            max_levels: MAX_LEVELS,
            max_leaves: MAX_LEVELS
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.root
    }
}
