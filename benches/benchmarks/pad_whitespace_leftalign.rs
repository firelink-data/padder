use criterion::{black_box, criterion_group, Criterion};
use padder::whitespace;
use padder::Alignment;

pub fn pad_whitespace_10_leftalign(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("pad ws 10 la", |b| {
        b.iter(|| black_box(whitespace("hej", width, Alignment::Left).unwrap()))
    });
}

pub fn pad_whitespace_100_leftalign(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("pad ws 100 la", |b| {
        b.iter(|| black_box(whitespace("bingbong", width, Alignment::Left).unwrap()))
    });
}

pub fn pad_whitespace_1000_leftalign(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("pad ws 1000 la", |b| {
        b.iter(|| {
            black_box(whitespace("Undercity is a cool capital...", width, Alignment::Left).unwrap())
        })
    });
}

pub fn pad_whitespace_10000_leftalign(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("pad ws 10000 la", |b| {
        b.iter(|| {
            black_box(
                whitespace(
                    "¤)(åäöåa this is a very long string... xd",
                    width,
                    Alignment::Left,
                )
                .unwrap(),
            )
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
