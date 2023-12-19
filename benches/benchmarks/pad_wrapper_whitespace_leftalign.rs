use criterion::{black_box, criterion_group, Criterion};
use padder::*;

pub fn pad_wrapper_whitespace_10_leftalign(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("pad wrapper ws 10 la", |b| {
        b.iter(|| black_box(pad("hej", width, Alignment::Left, Symbol::Whitespace)))
    });
}

pub fn pad_wrapper_whitespace_100_leftalign(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("pad wrapper ws 100 la", |b| {
        b.iter(|| black_box(pad("bingbong", width, Alignment::Left, Symbol::Whitespace)))
    });
}

pub fn pad_wrapper_whitespace_1000_leftalign(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("pad wrapper ws 1000 la", |b| {
        b.iter(|| {
            black_box(pad(
                "Undercity is a cool capital...",
                width,
                Alignment::Left,
                Symbol::Whitespace,
            ))
        })
    });
}

pub fn pad_wrapper_whitespace_10000_leftalign(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("pad wrapper ws 10000 la", |b| {
        b.iter(|| {
            black_box(pad(
                "¤)(åäöåa this is a very long string... xd",
                width,
                Alignment::Left,
                Symbol::Whitespace,
            ))
        })
    });
}

criterion_group!(
    pads,
    pad_wrapper_whitespace_10_leftalign,
    pad_wrapper_whitespace_100_leftalign,
    pad_wrapper_whitespace_1000_leftalign,
    pad_wrapper_whitespace_10000_leftalign,
);
