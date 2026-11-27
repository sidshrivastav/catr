# catr

A Rust implementation of the Unix `cat` command. It copies files and standard input to standard output in argument order.

## Usage

```sh
cargo run -- first.txt second.txt
printf 'from stdin\n' | cargo run -- first.txt -
```

Pass `-` to read standard input at that position. With no arguments, `catr` reads standard input by default.

## Development

`src/main.rs` defines the command-line arguments and connects the process streams. `src/lib.rs` handles concatenation and accepts any reader and writer, so the copy logic can be exercised with in-memory streams.

```sh
cargo build --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```
