use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::env;

#[path = "../src/dfs.rs"]
mod dfs;
use dfs::dfs;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tree 20 .", |b| {
        let current_dir = env::current_dir().unwrap();

        let args: Vec<String> = env::args().collect();
        let default = String::from("");
        let arg = args.get(1).unwrap_or(&default);

        let mut cache = HashMap::<String, String>::new();
        let root = current_dir.clone().join(arg);
        b.iter(|| dfs(black_box(root.clone()), black_box(&mut cache)))
    });
    c.bench_function("tree 20 next.js", |b| {
        let current_dir = env::current_dir().unwrap();

        let args: Vec<String> = env::args().collect();
        let default = String::from("../next.js");
        let arg = args.get(1).unwrap_or(&default);

        let mut cache = HashMap::<String, String>::new();
        let root = current_dir.clone().join(arg);
        b.iter(|| dfs(black_box(root.clone()), black_box(&mut cache)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
