use std::fs;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use myjson::lexer::lexer;
use myjson::{parse, parse_bytes, stringify};

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

fn byte_parser_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("byte_parse");
    let canada = fs::read_to_string("data/canada.json").unwrap();
    group.throughput(Throughput::Bytes(canada.len() as u64));
    group.bench_function("canada.json", |b| {
        b.iter(|| parse_bytes(black_box(canada.as_bytes())));
    });
    let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    group.throughput(Throughput::Bytes(citm_catalog.len() as u64));
    group.bench_function("citm_catalog.json", |b| {
        b.iter(|| parse_bytes(black_box(citm_catalog.as_bytes())));
    });
    let twitter = fs::read_to_string("data/twitter.json").unwrap();
    group.throughput(Throughput::Bytes(twitter.len() as u64));
    group.bench_function("twitter.json", |b| {
        b.iter(|| parse_bytes(black_box(twitter.as_bytes())));
    });
}

fn simd_parse_reference(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd");
    let mut canada = fs::read_to_string("data/canada.json").unwrap();
    group.throughput(Throughput::Bytes(canada.len() as u64));
    group.bench_function("canada.json", |b| unsafe {
        b.iter(|| simd_json::to_owned_value(canada.as_bytes_mut()));
    });
    let mut citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
    group.throughput(Throughput::Bytes(citm_catalog.len() as u64));
    group.bench_function("citm_catalog.json", |b| unsafe {
        b.iter(|| simd_json::to_owned_value(citm_catalog.as_bytes_mut()));
    });
    let mut twitter = fs::read_to_string("data/twitter.json").unwrap();
    group.throughput(Throughput::Bytes(twitter.len() as u64));
    group.bench_function("twitter.json", |b| unsafe {
        b.iter(|| simd_json::to_owned_value(twitter.as_bytes_mut()));
    });
}

fn serde_parse_reference(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("serde_from_str");
        let canada = fs::read_to_string("data/canada.json").unwrap();
        group.throughput(Throughput::Bytes(canada.len() as u64));
        group.bench_function("canada.json", |b| {
            b.iter(|| serde_json::from_str::<serde_json::Value>(&canada));
        });
        let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
        group.throughput(Throughput::Bytes(citm_catalog.len() as u64));
        group.bench_function("citm_catalog.json", |b| {
            b.iter(|| serde_json::from_str::<serde_json::Value>(&citm_catalog));
        });
        let twitter = fs::read_to_string("data/twitter.json").unwrap();
        group.throughput(Throughput::Bytes(twitter.len() as u64));
        group.bench_function("twitter.json", |b| {
            b.iter(|| serde_json::from_str::<serde_json::Value>(&twitter));
        });
    }

    {
        let mut group = c.benchmark_group("serde_to_str");
        let canada = fs::read_to_string("data/canada.json").unwrap();
        let canada = serde_json::from_str::<serde_json::Value>(&canada).unwrap();
        group.throughput(Throughput::Bytes(canada.to_string().len() as u64));
        group.bench_function("canada.json", |b| {
            b.iter(|| canada.to_string());
        });
        let citm_catalog = fs::read_to_string("data/citm_catalog.json").unwrap();
        let citm_catalog = serde_json::from_str::<serde_json::Value>(&citm_catalog).unwrap();
        group.throughput(Throughput::Bytes(citm_catalog.to_string().len() as u64));
        group.bench_function("citm_catalog.json", |b| {
            b.iter(|| citm_catalog.to_string());
        });
        let twitter = fs::read_to_string("data/twitter.json").unwrap();
        let twitter = serde_json::from_str::<serde_json::Value>(&twitter).unwrap();
        group.throughput(Throughput::Bytes(twitter.to_string().len() as u64));
        group.bench_function("twitter.json", |b| {
            b.iter(|| twitter.to_string());
        });
    }
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

criterion_group!(benches, lexer_benchmark, parser_benchmark, stringify_benchmark, simd_parse_reference, serde_parse_reference, byte_parser_benchmark);
criterion_main!(benches);