use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use smt::sparse_merkle_tree::SparseMerkleTree;

fn sparse_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sparse_insert");

    for size in [100, 500, 1_000, 5_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", size), size, |b, &size| {
            b.iter(|| {
                let mut tree = SparseMerkleTree::new();
                for i in 0..size {
                    let key = [i as u8; 32];
                    let value = [(i * 2) as u8; 32];
                    tree.insert(key, value);
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, sparse_insert_benchmark);
criterion_main!(benches);
