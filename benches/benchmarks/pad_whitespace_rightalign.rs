use criterion::{black_box, criterion_group, Criterion};
use padder::*;

pub fn pad_whitespace_10_rightalign(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("pad ws 10 ra", |b| {
        b.iter(|| black_box("hej".pad(width, Alignment::Right, Symbol::Whitespace)))
    });
}

pub fn pad_whitespace_100_rightalign(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("pad ws 100 ra", |b| {
        b.iter(|| black_box("longgggg".pad(width, Alignment::Right, Symbol::Whitespace)))
    });
}

pub fn pad_whitespace_1000_rightalign(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("pad ws 1000 ra", |b| {
        b.iter(|| {
            black_box("more !!3131 long haha 9481748718".pad(
                width,
                Alignment::Right,
                Symbol::Whitespace,
            ))
        })
    });
}

pub fn pad_whitespace_10000_rightalign(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("pad ws 10000 ra", |b| {
        b.iter(|| {
            black_box("this181874817481 is 178#(/!(#¤ super long string".pad(
                width,
                Alignment::Right,
                Symbol::Whitespace,
            ))
        })
    });
}

criterion_group!(
    pads,
    pad_whitespace_10_rightalign,
    pad_whitespace_100_rightalign,
    pad_whitespace_1000_rightalign,
    pad_whitespace_10000_rightalign,
);
