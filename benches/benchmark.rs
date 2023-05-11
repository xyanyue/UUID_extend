use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple_unsafe_uuid::UUID;

fn encode(i: u32) {
    // let mut uuid = UUID::new();
    for n in 0..100 {
        UUID::uuid_string(i + n);
        // println!("{} - {:?} - {}", i, encode.clone(), UUID::decode(encode));
    }
}
fn decode(c: String) {
    // let mut uuid = UUID::new();
    for _ in 0..100 {
        UUID::decode(c.clone());
        // println!("{} - {:?} - {}", i, encode.clone(), UUID::decode(encode));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("encode 2313131", |b| b.iter(|| encode(black_box(2313131))));
    c.bench_function("decode", |b| {
        b.iter(|| decode(black_box("AAAAAGRcP7pNjAG6AbpN7w".to_owned())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
