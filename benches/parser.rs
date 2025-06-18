use std::fs;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use myjson::parse;

fn parser_benchmark(c: &mut Criterion) {
    let canada = fs::read_to_string("data/canada.json").unwrap();
    c.bench_function("parse canada.json", |b| {
        b.iter(|| parse(black_box(canada.chars())));
    });
    let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    c.bench_function("parse citm_catalog.json", |b| {
        b.iter(|| parse(black_box(citm_catalog.chars())));
    });
    let twitter = fs::read_to_string("data/twitter.json").unwrap();
    c.bench_function("parse twitter.json", |b| {
        b.iter(|| parse(black_box(twitter.chars())));
    });
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);