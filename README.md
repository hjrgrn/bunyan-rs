<h1 align="center">bunyan-rs</h1>
<div align="center">
 <strong>
   A Rust port of <a href="https://github.com/trentm/node-bunyan" target="_blank">node-bunyan</a>, forked from <a href="https://github.com/LukeMathWalker/bunyan" target="_blank">bunyan</a>.
 </strong>
</div>

<br />


# Table of Contents
0. [How to install](#how-to-install)
1. [How to use](#how-to-use)
2. [Limitations](#limitations)
3. [Bunyan ecosystem in Rust](#bunyan-ecosystem-in-rust)
4. [Benchmarks](#benchmarks)
5. [License](#license)

## How to install

Using `cargo`:
```bash
git clone 'https://github.com/hjrgrn/bunyan-rs'
cd ./bunyan-rs
cargo install --path .
```

You can verify your installation with
```bash
bunyan --help
```


## How to use

You can pipe the output of a long-running job into it:
```bash
# Tail logs from a Docker container
docker logs -f my-app | bunyan

# Tail logs from a Kubernetes pod using kubectl
kubectl logs -f my-app-asdadf-cvcvcv

# Tail logs from a group of Kubernetes pods using stern
stern "my-app" --output raw --tail 100 | bunyan
```

Or you can use a file as input:
```bash
bunyan tests/all/corpus/all.log
```


## Comparisons

Compared to the original `bunyan` CLI, `bunyan-rs`:

- Does not support log snooping via DTrace (`-p` argument);
- Does not support the `-c/--condition` filtering mechanism;
- Does not support the `--pager/--no-pager` flags;
- Only supports UTC format for time.

Some of the above might or might not be added in the future.

Compared to the original [bunyan](https://github.com/LukeMathWalker/bunyan) `bunyan-rs`:

- supports files as input source
- supports multiple formatting styles.


## Bunyan ecosystem in Rust

You are writing a Rust application and you'd like to emit logs in bunyan format - what can you use?

Check out the following crates:

- [`tracing-bunyan-formatter`](https://crates.io/crates/tracing-bunyan-formatter), a bunyan formatter for [`tracing`](https://crates.io/crates/tracing);
- [`slog-bunyan`](https://crates.io/crates/slog-bunyan), a bunyan formatter for [`slog`](https://crates.io/crates/slog).


## Benchmarks

To benchmark `bunyan-rs` against the original NodeJS `bunyan` follow these steps:

- Build `bunyan-rs` using the `release` profile:
```bash
cargo build --release
```
- Install `bunyan` via `npm`. You will need `npx` as well;
- Benchmark!
```bash
# bunyan JS
time ./benchmark_js.sh
# bunyan-rs
time ./benchmark_rs.sh
```

The Rust code is highly non-optimised (we are allocating freely and wastefully!).


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
