use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use smt::sparse_merkle_tree::SparseMerkleTree;

fn sparse_proof_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sparse_proof");

    for size in [100, 1_000, 5_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::new("get_proof", size), size, |b, size| {
            let mut tree = SparseMerkleTree::new();
            for i in 0..*size {
                let key = [i as u8; 32];
                let value = [(i * 2) as u8; 32];
                tree.insert(key, value);
            }
            
            let test_key = [42 as u8; 32];
            let test_value = [84 as u8; 32];
            tree.insert(test_key, test_value);

            b.iter(|| {
                tree.get_proof(test_key)
            });
        });

        group.bench_with_input(BenchmarkId::new("verify_inclusion", size), size, |b, size| {
            let mut tree = SparseMerkleTree::new();
            for i in 0..*size {
                let key = [i as u8; 32];
                let value = [(i * 2) as u8; 32];
                tree.insert(key, value);
            }
            
            let test_key = [42 as u8; 32];
            let test_value = [84 as u8; 32];
            tree.insert(test_key, test_value);

            b.iter(|| {
                tree.verify_inclusion(test_key, test_value)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, sparse_proof_benchmark);
criterion_main!(benches);
