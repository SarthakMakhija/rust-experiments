use criterion::{Criterion, criterion_group, criterion_main};
use rust_lifetimes::serde::constant_size_struct::Node;

fn serialize_fixed_size(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("serialize_fixed_size");

    group.bench_function(
        "serialize fixed size node",
        |bencher| {
            bencher.iter(|| {
                let node = Node::new();
                let _ = node.serialize();
            } )
        });

    group.bench_function(
        "custom serialize fixed size node",
        |bencher| {
            bencher.iter(|| {
                let node = Node::new();
                let _ = node.custom_serialize();
            } )
        });
}

criterion_group!(benches, serialize_fixed_size);
criterion_main!(benches);