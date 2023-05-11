use criterion::{black_box, criterion_group, criterion_main, Criterion};
use UUID_extend::UUID;

fn encode(i: u32) {
    UUID::uuid_string(i);
}
fn decode(c: String) {
    UUID::decode(c);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("encode", |b| b.iter(|| encode(black_box(21333))));
    c.bench_function("decode", |b| {
        b.iter(|| decode(black_box("ZFycveL0AAoACuKu".to_owned())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
