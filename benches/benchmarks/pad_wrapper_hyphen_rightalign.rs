use criterion::{black_box, criterion_group, Criterion};
use padder::*;

pub fn pad_wrapper_hyphen_10_rightalign(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("pad wrapper hyphen 10 ra", |b| {
        b.iter(|| black_box(pad("hej", width, Alignment::Right, Symbol::Hyphen)))
    });
}

pub fn pad_wrapper_hyphen_100_rightalign(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("pad wrapper hyphen 100 ra", |b| {
        b.iter(|| black_box(pad("bingbong", width, Alignment::Right, Symbol::Hyphen)))
    });
}

pub fn pad_wrapper_hyphen_1000_rightalign(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("pad wrapper hyphen 1000 ra", |b| {
        b.iter(|| {
            black_box(pad(
                "Undercity is a cool capital...",
                width,
                Alignment::Right,
                Symbol::Hyphen,
            ))
        })
    });
}

pub fn pad_wrapper_hyphen_10000_rightalign(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("pad wrapper hyphen 10000 ra", |b| {
        b.iter(|| {
            black_box(pad(
                "¤)(åäöåa this is a very long string... xd",
                width,
                Alignment::Right,
                Symbol::Hyphen,
            ))
        })
    });
}

criterion_group!(
    pads,
    pad_wrapper_hyphen_10_rightalign,
    pad_wrapper_hyphen_100_rightalign,
    pad_wrapper_hyphen_1000_rightalign,
    pad_wrapper_hyphen_10000_rightalign,
);
