use criterion::{criterion_group, criterion_main, Criterion};

mod parser;
mod analysis;

fn run_benches(c: &mut Criterion) {
    parser::bench_parsing(c);
    analysis::bench_analysis(c);
}

criterion_group!(benches, run_benches);
criterion_main!(benches);
