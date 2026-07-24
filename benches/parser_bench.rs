use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dex_parser_rust::DexParser;
use dex_parser_rust::dex::apk::ApkParser;
use std::fs;

fn bench_parser(c: &mut Criterion) {
    let buffer = fs::read("workspace/sample/classes.dex").expect("Failed to read sample dex");

    c.bench_function("parse_classes_dex", |b| {
        b.iter(|| {
            let dex = DexParser::parse(black_box(&buffer)).expect("Failed to parse");
            black_box(dex);
        })
    });
}

fn bench_apk_parser(c: &mut Criterion) {
    let buffer = fs::read("workspace/sample/UnCrackable-Level1.apk").expect("Failed to read sample apk");

    c.bench_function("parse_uncrackable_apk", |b| {
        b.iter(|| {
            let apk = ApkParser::parse_apk(black_box(&buffer)).expect("Failed to parse APK");
            black_box(apk);
        })
    });
}

criterion_group!(benches, bench_parser, bench_apk_parser);
criterion_main!(benches);
