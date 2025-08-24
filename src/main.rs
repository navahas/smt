mod sparse_merkle_tree;
use sparse_merkle_tree::*;

mod utils;
use utils::*;

fn main() {
    let tree = SparseMerkleTree::new();
    let root = to_hex_prefixed(&tree.root());
    println!("{}", root);
}
