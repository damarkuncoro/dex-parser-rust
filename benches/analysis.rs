use criterion::{black_box, Criterion};
use dex_parser_rust::DexParser;
use dex_parser_rust::dex::analysis::XrefBuilder;
use std::fs;

pub fn bench_analysis(c: &mut Criterion) {
    let buffer = fs::read("workspace/sample/classes.dex").expect("Failed to read sample dex");
    let dex = DexParser::parse(&buffer).expect("Initial parse failed");

    let mut group = c.benchmark_group("DEX Analysis");

    group.bench_function("xref_builder", |b| {
        b.iter(|| {
            let xrefs = XrefBuilder::build(black_box(&dex.class_defs));
            black_box(xrefs);
        })
    });

    group.finish();
}
