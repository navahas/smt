mod sparse_merkle_tree;
use sparse_merkle_tree::*;

mod utils;
use utils::*;

use std::time::Instant;

fn main() {
    let mut tree = SparseMerkleTree::new();
    //let root = to_hex_prefixed(&tree.root());
    //println!("Initial root: {}", root);

    let batch_sizes = [10, 100, 1000, 10000, 100000];
    
    for &batch_size in &batch_sizes {
        
        for i in 0..batch_size {
            let key = keccak(&(i as u64).to_le_bytes());
            let value = keccak(&format!("value_{}", i).as_bytes());
            let start = Instant::now();
            tree.insert(key, value);
            let duration = start.elapsed();
            println!("[batch] {} | [i]: {} - {:?}", batch_size, i, duration);
        }
        
        //let root = to_hex_prefixed(&tree.root());
        //println!("Batch {}: - Root: {}", batch_size, &root[..18]);
        //println!("Batch {}: {:?} - Root: {}", batch_size, duration, &root[..18]);
        
    }
}
