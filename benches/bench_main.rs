use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::format_whitespace_leftalign::formats,
    benchmarks::pad_whitespace_leftalign::pads,
}
