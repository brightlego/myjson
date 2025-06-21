use std::fs;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use myjson::lexer::lexer;
use myjson::{parse, stringify};

fn lexer_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lex");
    let canada = fs::read_to_string("data/canada.json").unwrap();
    group.throughput(Throughput::Bytes(canada.len() as u64));
    group.bench_function("canada.json", |b| {
        b.iter(|| lexer(black_box(canada.chars())).last());
    });
    let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    group.throughput(Throughput::Bytes(citm_catalog.len() as u64));
    group.bench_function("citm_catalog.json", |b| {
        b.iter(|| lexer(black_box(citm_catalog.chars())).last());
    });
    let twitter = fs::read_to_string("data/twitter.json").unwrap();
    group.throughput(Throughput::Bytes(twitter.len() as u64));
    group.bench_function("twitter.json", |b| {
        b.iter(|| lexer(black_box(twitter.chars())).last());
    });
}

fn parser_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    let canada = fs::read_to_string("data/canada.json").unwrap();
    group.throughput(Throughput::Bytes(canada.len() as u64));
    group.bench_function("canada.json", |b| {
        b.iter(|| parse(black_box(canada.chars())));
    });
    let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    group.throughput(Throughput::Bytes(citm_catalog.len() as u64));
    group.bench_function("citm_catalog.json", |b| {
        b.iter(|| parse(black_box(citm_catalog.chars())));
    });
    let twitter = fs::read_to_string("data/twitter.json").unwrap();
    group.throughput(Throughput::Bytes(twitter.len() as u64));
    group.bench_function("twitter.json", |b| {
        b.iter(|| parse(black_box(twitter.chars())));
    });
}

fn stringify_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stringify");
    let canada = fs::read_to_string("data/canada.json").unwrap();
    let canada = parse(canada.chars()).unwrap();
    group.throughput(Throughput::Bytes(stringify(&canada).len() as u64));
    group.bench_function("canada.json", |b| {
        b.iter(|| stringify(black_box(&canada)));
    });
    let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    let citm_catalog = parse(citm_catalog.chars()).unwrap();
    group.throughput(Throughput::Bytes(stringify(&citm_catalog).len() as u64));
    group.bench_function("citm_catalog.json", |b| {
        b.iter(|| stringify(black_box(&citm_catalog)));
    });
    let twitter = fs::read_to_string("data/twitter.json").unwrap();
    let twitter = parse(twitter.chars()).unwrap();
    group.throughput(Throughput::Bytes(stringify(&twitter).len() as u64));
    group.bench_function("twitter.json", |b| {
        b.iter(|| stringify(black_box(&twitter)));
    });
}

criterion_group!(benches, lexer_benchmark, parser_benchmark, stringify_benchmark);
criterion_main!(benches);