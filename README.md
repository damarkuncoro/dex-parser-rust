# DEX Parser Rust

A high-performance, modular, and parallel Android DEX file parser written in Rust. This tool is designed as a modern alternative to the traditional `dexdump` utility, offering better performance and structured output.

## Features

- **🚀 High Performance**: Built with Rust for maximum speed and memory safety.
- **⚡ Parallel Parsing**: Utilizes multi-core processing via [Rayon](https://github.com/rayon-rs/rayon) to parse large DEX files exponentially faster than `dexdump`.
- **🧩 Modular Architecture**: Designed with **Dependency Injection** and **Strategy Patterns** for easy extensibility.
- **📊 Structured Output**: Supports both traditional text output and machine-readable **JSON**.
- **🧪 Testable**: Includes unit tests with Mock Resolvers for robust development.
- **🛠 Professional CLI**: Powered by `clap` with full help, versioning, and validation support.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone <your-repo-url>
cd "DEX Parser Rust"
cargo build --release
```

## Usage

### Basic Text Output (Dexdump Style)
```bash
cargo run -- path/to/classes.dex
```

### JSON Output
```bash
cargo run -- path/to/classes.dex --format json
```

### Help
```bash
cargo run -- --help
```

## Architecture

The project is divided into several clear modules:
- `parsers`: Multi-stage parsing logic (Header, Metadata, Classes).
- `instructions`: Opcode decoding and symbol resolution.
- `display`: Flexible output strategies (Text/JSON).
- `models`: Strongly typed DEX structures.
- `traits`: Dependency Injection interfaces for symbol resolution.

## License
MIT / Apache-2.0
