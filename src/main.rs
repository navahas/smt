mod sparse_merkle_tree;
use sparse_merkle_tree::*;

mod utils;

fn main() {
    let tree = SparseMerkleTree::new().init();
    println!("{:?}", tree);
}
