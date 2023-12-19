use criterion::{black_box, criterion_group, Criterion};
use padder::*;

pub fn pad_whitespace_10_leftalign(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("pad ws 10 la", |b| {
        b.iter(|| black_box("hej".pad(width, Alignment::Left, Symbol::Whitespace)))
    });
}

pub fn pad_whitespace_100_leftalign(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("pad ws 100 la", |b| {
        b.iter(|| black_box("bingbong".pad(width, Alignment::Left, Symbol::Whitespace)))
    });
}

pub fn pad_whitespace_1000_leftalign(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("pad ws 1000 la", |b| {
        b.iter(|| {
            black_box("Undercity is a cool capital...".pad(
                width,
                Alignment::Left,
                Symbol::Whitespace,
            ))
        })
    });
}

pub fn pad_whitespace_10000_leftalign(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("pad ws 10000 la", |b| {
        b.iter(|| {
            black_box("¤)(åäöåa this is a very long string... xd".pad(
                width,
                Alignment::Left,
                Symbol::Whitespace,
            ))
        })
    });
}

criterion_group!(
    pads,
    pad_whitespace_10_leftalign,
    pad_whitespace_100_leftalign,
    pad_whitespace_1000_leftalign,
    pad_whitespace_10000_leftalign,
);
