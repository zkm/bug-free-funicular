# bug-free-funicular

[![Rust CI](https://github.com/zkm/bug-free-funicular/actions/workflows/ci.yml/badge.svg)](https://github.com/zkm/bug-free-funicular/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

![Rust 2024 edition](https://img.shields.io/badge/Rust-2024%20edition-000000?logo=rust&logoColor=white)
![rustfmt and clippy enforced](https://img.shields.io/badge/rustfmt%20%2B%20clippy-enforced-DEA584)

A tiny Rust CLI to learn the basics by doing. It includes a few small commands and unit tests.

## Quick start

Build and run tests:

```
cargo test
```

Run the CLI:

```
cargo run -- <command> [args]
```

## Commands

- hello <name> [--yell]
	- Greet someone, optionally in ALL CAPS.
- sum <n1> <n2> ...
	- Sum a list of integers and print the total.
- guess
	- Play a number guessing game (1..=100).
- read <path>
	- Read a file and print a short summary.

Examples:

```
cargo run -- hello Alice
cargo run -- hello Bob --yell
cargo run -- sum 4 10 -3
cargo run -- guess
cargo run -- read README.md
```

## Project layout

- `src/main.rs` — CLI entrypoint and routing to commands.
- `src/commands/` — small command implementations:
	- `hello.rs`, `sum.rs`, `guess.rs`, `read.rs`
- `src/lib.rs` — a tiny library (`math::sum`) with unit tests.

## Learnings you can take from here

- Parsing CLI args with `std::env::args`
- Modules and submodules (`mod`, `pub mod`)
- Error handling with `Result<T, Box<dyn Error>>`
- Reading stdin/stdout and files
- Using crates: `rand` (runtime) and `tempfile` (test)
- Writing and running tests (`#[cfg(test)]` / `cargo test`)

## License

MIT