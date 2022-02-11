use criterion::{criterion_group, criterion_main, Criterion};
use joyful_minigrep::core::{search, search_insensitive};

fn search_benchmark(c: &mut Criterion) {
    c.bench_function("sensitive", |b| {
        b.iter(|| {
            let file = std::fs::read_to_string("poem.txt").unwrap();
            search("body", &file);
        })
    });
}

fn insensitive_search_benchmark(c: &mut Criterion) {
    c.bench_function("sensitive", |b| {
        b.iter(|| {
            let file = std::fs::read_to_string("poem.txt").unwrap();
            search_insensitive("body", &file);
        })
    });
}

criterion_group!(benches, search_benchmark, insensitive_search_benchmark);
criterion_main!(benches);
