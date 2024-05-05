use criterion::{black_box, criterion_group, Criterion};
use padder::*;

pub fn pad_and_push_to_buffer_wrapper_whitespace_10_center(c: &mut Criterion) {
    let width: usize = 10;
    let mut buffer: Vec<u8> = Vec::with_capacity(width);
    c.bench_function("pad&push wrapper ws 10 center", |b| {
        b.iter(|| black_box({
            pad_and_push_to_buffer("hej".as_bytes(), width, Alignment::Center, Symbol::Whitespace, &mut buffer);
        }))
    });
}

pub fn pad_and_push_to_buffer_wrapper_whitespace_100_center(c: &mut Criterion) {
    let width: usize = 100;
    let mut buffer: Vec<u8> = Vec::with_capacity(width);
    c.bench_function("pad&push wrapper ws 100 center", |b| {
        b.iter(|| black_box({
            pad_and_push_to_buffer("uga78r9eguerbknma bba re7".as_bytes(), width, Alignment::Center, Symbol::Whitespace, &mut buffer);
        }))
    });
}

pub fn pad_and_push_to_buffer_wrapper_whitespace_1000_center(c: &mut Criterion) {
    let width: usize = 1000;
    let mut buffer: Vec<u8> = Vec::with_capacity(width);
    c.bench_function("pad&push wrapper ws 1000 center", |b| {
        b.iter(|| {
            black_box({
                pad_and_push_to_buffer(
                    "Undercity is a cool capital...".as_bytes(),
                    width,
                    Alignment::Left,
                    Symbol::Whitespace,
                    &mut buffer,
                )
            })
        })
    });
}

pub fn pad_and_push_to_buffer_wrapper_whitespace_10000_center(c: &mut Criterion) {
    let width: usize = 10000;
    let mut buffer: Vec<u8> = Vec::with_capacity(width);
    c.bench_function("pad&push wrapper ws 10000 center", |b| {
        b.iter(|| {
            black_box({
                pad_and_push_to_buffer(
                    "¤)(åäöåa this is a very long string... xd".as_bytes(),
                    width,
                    Alignment::Left,
                    Symbol::Whitespace,
                    &mut buffer,
                )
            })
        })
    });
}

criterion_group!(
    pads,
    pad_and_push_to_buffer_wrapper_whitespace_10_center,
    pad_and_push_to_buffer_wrapper_whitespace_100_center,
    pad_and_push_to_buffer_wrapper_whitespace_1000_center,
    pad_and_push_to_buffer_wrapper_whitespace_10000_center,
);
