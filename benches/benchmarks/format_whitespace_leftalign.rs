use criterion::{black_box, criterion_group, Criterion};

pub fn format_whitespace_10_leftalign(c: &mut Criterion) {
    let width: usize = 10;
    c.bench_function("format! ws 10 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "hej")))
    });
}

pub fn format_whitespace_100_leftalign(c: &mut Criterion) {
    let width: usize = 100;
    c.bench_function("format! ws 100 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "bingbong")))
    });
}

pub fn format_whitespace_1000_leftalign(c: &mut Criterion) {
    let width: usize = 1000;
    c.bench_function("format! ws 1000 la", |b| {
        b.iter(|| black_box(format!("{:<width$}", "Undercity is a cool capital...")))
    });
}

pub fn format_whitespace_10000_leftalign(c: &mut Criterion) {
    let width: usize = 10000;
    c.bench_function("format! ws 10000 la", |b| {
        b.iter(|| {
            black_box(format!(
                "{:<width$}",
                "¤)(åäöåa this is a very long string... xd"
            ))
        })
    });
}

criterion_group!(
    formats,
    format_whitespace_10_leftalign,
    format_whitespace_100_leftalign,
    format_whitespace_1000_leftalign,
    format_whitespace_10000_leftalign,
);
