#![feature(test)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, io};
use walkdir::WalkDir;

static SPACE: &str = "    ";
static BRANCH: &str = "│   ";
static STEM: &str = "├── ";
static CORNER: &str = "└── ";

/// Run `rustup default nightly` to get the nightly toolchain.
/// Run `rustup default stable` to get the stable toolchain.
///
/// Run `cargo test -- --test-threads=1` to run tests in parallel.
/// Run `cargo bench` to run benchmarks.
///
/// Run `cargo run` to run the program.
fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;

    let args: Vec<String> = env::args().collect();
    let default = String::from("");
    let arg = args.get(1).unwrap_or(&default);

    let mut cache = HashMap::<String, String>::new();
    let root = current_dir.clone().join(arg);

    dfs(root, &mut cache);

    Ok(())
}

fn dfs(root: PathBuf, cache: &mut HashMap<String, String>) {
    for entry in WalkDir::new(root.clone())
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().unwrap_or("").starts_with("."))
        .filter_map(Result::ok)
        .filter(|e| e.path() != root)
    {
        let absolute_path = entry.path();
        let relative_path = absolute_path.strip_prefix(root.as_path()).unwrap();

        let display_string = relative_path.display().to_string();
        let parts = display_string.split("/").collect::<Vec<&str>>();

        let mut abs = root.clone();
        let mut res = String::from("");

        // draw the parts
        let parts_len = parts.iter().count();
        for (i, part) in parts.iter().enumerate() {
            abs = abs.join(part);
            let is_base = i == parts_len - 1;

            let dir = abs.clone().parent().unwrap().read_dir().unwrap();
            let iter = dir.enumerate();
            let len = abs.clone().parent().unwrap().read_dir().unwrap().count();

            if is_base {
                for (i, f) in iter {
                    if *part == f.unwrap().file_name().to_str().unwrap() {
                        if i == len - 1 {
                            res += &CORNER;
                        } else {
                            res += &STEM;
                        }
                    }
                }
            } else {
                if cache.get(abs.to_str().unwrap()).is_some() {
                    // println!("CACHE HIT!");
                    res = cache.get(abs.to_str().unwrap()).unwrap().clone();
                } else {
                    for (i, f) in iter {
                        if *part == f.unwrap().file_name().to_str().unwrap() {
                            if i == len - 1 {
                                res += &SPACE;
                            } else {
                                res += &BRANCH;
                            }
                        }
                    }
                    cache.insert(abs.to_str().unwrap().to_string(), res.clone());
                }
            }
        }

        println!("{}{}", res, entry.file_name().to_str().unwrap());
    }
}

extern crate test;

/// http://seenaburns.com/benchmarking-rust-with-cargo-bench/
#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    // ❯ cargo bench
    // Finished bench [optimized] target(s) in 0.00s
    //  Running unittests (target/release/deps/rust_tree-e3f745aace8a21db)
    //
    // running 1 test
    // test tests::bench_main ... bench: 580,826,430 ns/iter (+/- 213,213,738)
    //
    // test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 174.10s
    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| {
            black_box({
                let current_dir = env::current_dir().unwrap();
                let root = current_dir.clone().join("");
                println!("{}", root.display());
                let mut cache = HashMap::<String, String>::new();
                dfs(root.clone(), &mut cache)
            });
        });
    }
}

// with caching
// test tests::bench_main ... bench: 413,285,189 ns/iter (+/- 41,656,455)
