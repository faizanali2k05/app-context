//! context verification for app-context.
// Criterion generates a public harness function in this private benchmark binary.
#![allow(missing_docs)]
use app_context::Context;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark(c: &mut Criterion) {
    let context = Context::builder().with(123_u64).unwrap().build();
    c.bench_function("get_arc", |b| {
        b.iter(|| black_box(context.get::<u64>().unwrap()))
    });
    c.bench_function("get_ref", |b| {
        b.iter(|| black_box(context.get_ref::<u64>().unwrap()))
    });
    c.bench_function("clone_context", |b| b.iter(|| black_box(context.clone())));
    c.bench_function("initialize_one_type", |b| {
        b.iter(|| black_box(Context::builder().with(black_box(123_u64)).unwrap().build()))
    });
    c.bench_function("scope_one_type", |b| {
        b.iter(|| black_box(context.scope().build()))
    });
    c.bench_function("initialize_four_types", |b| {
        b.iter(|| {
            black_box(
                Context::builder()
                    .with(black_box(1_u8))
                    .unwrap()
                    .with(2_u16)
                    .unwrap()
                    .with(3_u32)
                    .unwrap()
                    .with(4_u64)
                    .unwrap()
                    .build(),
            )
        })
    });
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
