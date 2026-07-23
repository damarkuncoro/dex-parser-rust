# Software Architecture Description (SAD) - ISO/IEC/IEEE 42010:2011 Compliant

## 1. Introduction
This document provides a formal architecture description for `dex-parser-rust`, a high-performance, memory-safe, and modular Android DEX/APK parsing engine. It is structured according to the ISO/IEC/IEEE 42010:2011 standard.

---

## 2. The Three Pillars of dex-parser-rust
The entire design of the system is built upon three foundational pillars:

### I. High Performance (Parallelism)
Leveraging modern multi-core hardware via **Rayon**. Unlike traditional single-threaded parsers, this engine parallelizes class processing, high-level linking, and bytecode disassembly, providing near-linear performance scaling with CPU core count.

### II. Safety & Efficiency (Zero-Copy)
Utilizing Rust's ownership model to implement **Zero-Copy parsing**. Data (Strings, Types, Signatures) is referenced directly from the original buffer, minimizing heap allocations and eliminating memory vulnerabilities.

### III. Radical Modularity (Atomic Parsing & Linking)
Strict separation between **Physical Binary Extraction** (Atomic Parsers) and **Logical Symbol Resolution** (Linker Engine). This "Shared-Nothing" architecture ensures that components are highly independent and testable.

---

## 3. Architecture Viewpoints

### 3.1 Logical Viewpoint
Focuses on the functional decomposition. The system is designed using the **Linker Pattern** and **Registry-based Opcodes**.

### 3.2 Process Viewpoint
Describes runtime behavior, specifically the multi-stage pipeline: **Extraction -> Validation -> Resolution -> Linking**.

### 3.3 Implementation Viewpoint
Focuses on the organization of software modules. Includes **Auto-Versioning** via build scripts and **Cross-Language Interoperability** via FFI.

---

## 4. Architecture Views

### 4.1 Logical View
- **`ValidationRule` Trait**: A rule-based engine for pluggable file integrity checks.
- **`DexResolver` Trait**: Dependency Injection interface for zero-copy symbol resolution.
- **`OpcodeTable`**: A modular reference for 100% of Dalvik opcodes.
- **Public API**: Ergonomic entry points like `DexParser::parse(buffer)` and `DexParser::parse_file(path)`.
- **C-ABI Export**: Stable FFI layer for integration with other languages.

### 4.2 Process View (Data Pipeline)
1. **Stage 1: Atomic Extraction**: Raw binary data is extracted into `RawModels`.
2. **Stage 2: Rule-Based Validation**: Sequential execution of integrity rules.
3. **Stage 3: Value Resolution**: Zero-copy resolution of descriptors.
4. **Stage 4: Parallel Linking & Assembly**: Orchestrating the `DexLinker` cross-threads.

### 4.3 Implementation View (Module Structure)
- **`readers/`**: Low-level binary stream management.
- **`parsers/`**: Atomic units with internal sub-modules.
- **`linker/`**: The central hub for connecting raw indices into logical relations.
- **`ffi.rs`**: The C-compatible bridge for external language support.
- **`build.rs`**: Automatic versioning script for injecting Git metadata.

### 4.4 Deployment View
- **APK/Multidex Support**: Integrated `ApkParser` for automatic ZIP extraction.
- **Polyglot Deployment**: Available as a standalone CLI, a Rust library (Crates.io), or a Shared Object library (`cdylib`) for Python/C++/Java.
- **CI/CD Pipeline**: Automated testing and cross-platform releases via GitHub Actions.

---

## 5. Architecture Rationale
- **Rust Language**: Chosen for memory safety and zero-cost abstractions.
- **Rayon**: Selected for high-level data parallelism.
- **C-ABI (FFI)**: Implemented to make the high-performance engine accessible to the wider developer ecosystem beyond Rust.
- **Git-linked Versioning**: Ensured for binary traceability.

---

## 6. Consistency and Completeness
- Compliant with ISO 42010 by addressing stakeholders through documented viewpoints.
