# RLOX
[![CI: Build, Test and Format](https://github.com/ID-UNCLEAR/rlox/actions/workflows/ci.yml/badge.svg)](https://github.com/ID-UNCLEAR/rlox/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/ID-UNCLEAR/rlox/graph/badge.svg?token=EBKZOOVXKZ)](https://codecov.io/gh/ID-UNCLEAR/rlox)

A Lox compiler written in Rust based on the book [Crafting Interpreters](https://craftinginterpreters.com) by Bob Nystrom.

## Progress
Currently working through: [Chapter 10. Functions](https://craftinginterpreters.com/functions.html#function-objects)

## Todos

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain recommended)
- `cargo` (included with Rust)

### Build
To build the project:

```bash
  cargo build --release
```

### Run
To run the interpreter:

```bash
  cargo run -- --path path/to/file.lox
```

Or enter interactive mode:

```bash
  cargo run
```

### Test
To run tests:

```bash
  cargo test
```

### Format
To format the code:

```bash
  cargo fmt
```

### Lint
To check for common mistakes:

```bash
  cargo clippy
```

### Supported Compilation Targets

This project aims to support the following platforms:

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-gnu`

> You can add and build for these targets with `rustup target add` and `cargo build --target <target>` if needed.
