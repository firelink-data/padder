use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::format_whitespace_leftalign::formats,
    benchmarks::pad_whitespace_leftalign::pads,
    benchmarks::pad_whitespace_rightalign::pads,
    benchmarks::pad_wrapper_hyphen_rightalign::pads,
    benchmarks::pad_wrapper_whitespace_leftalign::pads,
    benchmarks::pad_and_push_to_buffer_wrapper_whitespace_center::pads,
}
