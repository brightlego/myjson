use std::fs;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use myjson::lexer::lexer;

fn lexer_benchmark(c: &mut Criterion) {
    let canada = fs::read_to_string("data/canada.json").unwrap();
    c.bench_function("lex canada.json", |b| {
        b.iter(|| lexer(black_box(canada.chars())).last());
    });
    let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    c.bench_function("lex citm_catalog.json", |b| {
        b.iter(|| lexer(black_box(citm_catalog.chars())).last());
    });
    let twitter = fs::read_to_string("data/twitter.json").unwrap();
    c.bench_function("lex twitter.json", |b| {
        b.iter(|| lexer(black_box(twitter.chars())).last());
    });
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);