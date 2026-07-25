use criterion::{black_box, Criterion};
use dex_parser_rust::DexParser;
use std::fs;

pub fn bench_parsing(c: &mut Criterion) {
    let buffer = fs::read("workspace/sample/classes.dex").expect("Failed to read sample dex");

    let mut group = c.benchmark_group("DEX Parsing");

    group.bench_function("full_parse_classes_dex", |b| {
        b.iter(|| {
            let dex = DexParser::parse(black_box(&buffer)).expect("Failed to parse");
            black_box(dex);
        })
    });

    group.finish();
}
