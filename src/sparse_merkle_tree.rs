use sha3::{Digest, Keccak256};
use crate::utils;

const MAX_LEAVES: usize = 8;
const MAX_LEVELS: usize = 1 << MAX_LEAVES;

#[derive(Debug)]
pub struct SparseMerkleTree {
    leaves: Vec<Vec<u8>>,
    root: Option<Vec<u8>>,
    max_leaves: usize
}

impl SparseMerkleTree {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
            root: None,
            max_leaves: MAX_LEAVES
        }
    }
}
