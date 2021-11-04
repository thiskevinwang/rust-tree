use std::collections::HashMap;
use std::{env, io};

mod dfs;

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

    dfs::dfs(root, &mut cache);

    Ok(())
}
