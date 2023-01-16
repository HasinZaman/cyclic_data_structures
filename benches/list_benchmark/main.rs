use criterion::{criterion_group, criterion_main, Criterion};

mod generation;
mod drop_front;
mod drop_back;
mod reading;


criterion_group!{
    name = benches;
    config = Criterion::default()
        .sample_size(100);
    targets = reading::list_reading_benchmark, drop_back::list_drop_back_benchmark, drop_front::list_drop_front_benchmark, generation::list_generation_benchmark
}
criterion_main!(benches);