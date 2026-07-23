# dex-parser-rust 🚀

A high-performance, modular, and parallel Android DEX/APK parsing engine written in Rust. Designed as a modern, safe, and significantly faster alternative to traditional utilities like `dexdump`.

---

## 🌟 The Three Pillars

1. **High Performance (Parallelism)**: Leverage modern multi-core hardware via **Rayon**. Class processing, high-level linking, and disassembly are fully parallelized.
2. **Safety & Efficiency (Zero-Copy)**: Utilize Rust's ownership model for **Zero-Copy parsing**. Referencing data directly from the original buffer to minimize heap allocations and eliminate memory vulnerabilities.
3. **Radical Modularity (Atomic Parsing)**: Strict separation between physical binary extraction and logical symbol resolution through a dedicated **Linker Engine**.

---

## ✨ Features

- **📦 APK & Multidex Support**: Native support for extracting and parsing all `.dex` files inside an `.apk` (ZIP) container.
- **🔍 100% Opcode Coverage**: Comprehensive disassembly support for all Dalvik opcodes (0x00 - 0xFF), including modern ART-specific instructions.
- **🔗 Intelligent Linking**: Automatically resolves method signatures, field types, and class hierarchies into human-readable strings.
- **📝 Integrated Debug Info**: Inline support for `.line` and `.local` directives directly in the disassembly.
- **📊 Structured Export**: Traditional **Text** output for human auditing and **JSON** for automated tool chains.
- **🛡️ Robust Validation**: Rule-based engine to ensure file integrity (Magic, Checksum, Offsets, MapList).

---

## 🚀 Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) (1.70+) installed.

```bash
git clone https://github.com/damarkuncoro/dex-parser-rust
cd dex-parser-rust
cargo build --release
```

The binary will be available at `./target/release/dex-parser-rust`.

---

## 🛠 Library Usage (Rust)

Add `dex-parser-rust` to your `Cargo.toml`.

```rust
use dex_parser_rust::DexParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ergonomic Public API
    let dex = DexParser::parse_file("classes.dex")?;

    println!("DEX Checksum: {:08x}", dex.header.checksum);

    for class in &dex.class_defs {
        println!("Class #{}: {}", class.class_idx, class.name);
    }
    Ok(())
}
```

---

## 🌐 Cross-Language Integration

### 1. CLI + JSON (Universal)
Suitable for any language (Python, Node.js, Go).
```bash
./dex-parser-rust classes.dex --format json | jq .header.checksum
```

### 2. C-API / FFI (Maximum Performance)
`dex-parser-rust` can be compiled as a shared library (`.so`, `.dylib`, or `.dll`) for integration into C++, Python (via `ctypes`), or Java (via JNI).

Example (Python using `ctypes`):
```python
import ctypes
lib = ctypes.CDLL("./target/release/libdex_parser_rust.dylib")
count = lib.get_class_count(b"classes.dex")
print(f"Total classes: {count}")
```

---

## 📖 Documentation

- [**Software Architecture Description (SAD)**](./docs/ARCHITECTURE.md): ISO/IEC/IEEE 42010:2011 compliant document detailing the system's design.

---

## 📂 Project Structure

- `src/dex/readers`: Low-level binary stream management.
- `src/dex/parsers`: Atomic units for each DEX section.
- `src/dex/linker`: Central hub for logical symbol resolution.
- `src/dex/models`: Separation of physical (Raw) and high-level logical models.
- `src/dex/display`: Strategy-based formatters (Text/JSON).
- `src/ffi.rs`: Foreign Function Interface exports.

---

## 📜 License

This project is licensed under either the [MIT license](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE).
