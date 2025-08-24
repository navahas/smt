use std::collections::HashMap;
use crate::utils::*;

const MAX_LEVELS: usize = 32;
const MAX_LEAVES: usize = 1 << MAX_LEVELS; // bitwise for 2^x

#[derive(Debug)]
pub struct SparseMerkleTree {
    // key-value store for leaf nodes
    leaves: HashMap<[u8; 32], [u8; 32]>,
    // precomputed hashes per level
    zero: Vec<[u8; 32]>,
    root: [u8; 32],
    max_levels: usize,

}

impl SparseMerkleTree {
    pub fn new() -> Self {
        let zero_leaves = precompute_zero_hashes(MAX_LEVELS);
        let root = zero_leaves[MAX_LEVELS];
        Self {
            leaves: std::collections::HashMap::new(),
            zero: zero_leaves,
            root: root,
            max_levels: MAX_LEVELS,
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.root
    }

    pub fn insert(&mut self, key: [u8; 32], value: [u8; 32]) {
        self.leaves.insert(key, value);
        //self.root = self.compute_root();
    }

    pub fn verify_inclusion(&self, key: [u8; 32], value: [u8; 32]) -> bool {
        match self.leaves.get(&key) {
            Some(stored_value) => *stored_value == value,
            None => false,
        }
    }

    pub fn get_proof(&self, key: [u8; 32]) -> Option<Vec<[u8; 32]>> {
        if !self.leaves.contains_key(&key) {
            return None;
        }

        let path = self.key_to_path(&key);
        let mut proof = Vec::new();
        let mut current_level: HashMap<Vec<bool>, [u8; 32]> = HashMap::new();

        for (k, value) in &self.leaves {
            let k_path = self.key_to_path(k);
            current_level.insert(k_path, *value);
        }

        for level in (0..self.max_levels).rev() {
            let mut next_level: HashMap<Vec<bool>, [u8; 32]> = HashMap::new();
            let mut parent_children: HashMap<Vec<bool>, (Option<[u8; 32]>, Option<[u8; 32]>)> = HashMap::new();

            for (node_path, hash) in &current_level {
                if node_path.len() > level {
                    let parent_path = node_path[..level].to_vec();
                    let is_right = node_path[level];
                    
                    let entry = parent_children.entry(parent_path).or_insert((None, None));
                    if is_right {
                        entry.1 = Some(*hash);
                    } else {
                        entry.0 = Some(*hash);
                    }
                }
            }

            if level < path.len() {
                let current_path = path[..level].to_vec();
                if let Some((left, right)) = parent_children.get(&current_path) {
                    let sibling = if path[level] {
                        left.unwrap_or(self.zero[level + 1])
                    } else {
                        right.unwrap_or(self.zero[level + 1])
                    };
                    proof.push(sibling);
                } else {
                    proof.push(self.zero[level + 1]);
                }
            }

            for (parent_path, (left, right)) in parent_children {
                let left_hash = left.unwrap_or(self.zero[level + 1]);
                let right_hash = right.unwrap_or(self.zero[level + 1]);
                
                if left_hash != self.zero[level + 1] || right_hash != self.zero[level + 1] {
                    let parent_hash = hash_pair(left_hash, right_hash);
                    next_level.insert(parent_path, parent_hash);
                }
            }
            
            current_level = next_level;
        }

        Some(proof)
    }

    fn compute_root(&self) -> [u8; 32] {
        let mut current_level: HashMap<Vec<bool>, [u8; 32]> = HashMap::new();
        
        // Populate leaf level with actual values
        for (key, value) in &self.leaves {
            let path = self.key_to_path(key);
            current_level.insert(path, *value);
        }
        
        // Iteratively compute each level from bottom to top
        for level in (0..self.max_levels).rev() {
            let mut next_level: HashMap<Vec<bool>, [u8; 32]> = HashMap::new();
            
            // Group nodes by their parent path
            let mut parent_children: HashMap<Vec<bool>, (Option<[u8; 32]>, Option<[u8; 32]>)> = HashMap::new();
            
            for (path, hash) in &current_level {
                if path.len() > level {
                    let parent_path = path[..level].to_vec();
                    let is_right = path[level];
                    
                    let entry = parent_children.entry(parent_path).or_insert((None, None));
                    if is_right {
                        entry.1 = Some(*hash);
                    } else {
                        entry.0 = Some(*hash);
                    }
                }
            }
            
            // Compute parent hashes
            for (parent_path, (left, right)) in parent_children {
                let left_hash = left.unwrap_or(self.zero[level + 1]);
                let right_hash = right.unwrap_or(self.zero[level + 1]);
                
                // Only store non-zero hashes to maintain sparsity
                if left_hash != self.zero[level + 1] || right_hash != self.zero[level + 1] {
                    let parent_hash = hash_pair(left_hash, right_hash);
                    next_level.insert(parent_path, parent_hash);
                }
            }
            
            current_level = next_level;
        }
        
        // Return root hash or zero if no nodes at root level
        current_level.get(&vec![]).copied().unwrap_or(self.zero[0])
    }

    fn key_to_path(&self, key: &[u8; 32]) -> Vec<bool> {
        let mut path = Vec::with_capacity(self.max_levels);
        for i in 0..self.max_levels {
            let byte_index = i / 8;
            let bit_index = 7 - (i % 8);
            if byte_index < 32 {
                let bit = (key[byte_index] >> bit_index) & 1 == 1;
                path.push(bit);
            }
        }
        path
    }
}
