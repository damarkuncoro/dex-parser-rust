# Software Architecture Description (SAD) - ISO/IEC/IEEE 42010:2011 Compliant

## 1. Introduction
This document provides a formal architecture description for `dex-parser-rust`, a high-performance, memory-safe, and modular Android DEX/APK parsing engine. It is structured according to the ISO/IEC/IEEE 42010:2011 standard.

---

## 2. The Three Pillars of dex-parser-rust
The entire design of the system is built upon three foundational pillars:

### I. High Performance (Parallelism)
Leveraging modern multi-core hardware via **Rayon**. Unlike traditional single-threaded parsers, this engine parallelizes class processing, high-level linking, and bytecode disassembly, providing near-linear performance scaling with CPU core count.

### II. Safety & Efficiency (Zero-Copy)
Utilizing Rust's ownership and lifetime model to implement **Zero-Copy parsing**. Data (Strings, Types, Signatures) is referenced directly from the original buffer, minimizing heap allocations and eliminating memory vulnerabilities common in C/C++ implementations.

### III. Radical Modularity (Atomic Parsing & Linking)
Strict separation between **Physical Binary Extraction** (Atomic Parsers) and **Logical Symbol Resolution** (Linker Engine). This "Shared-Nothing" architecture ensures that components are highly independent, testable, and resilient to format changes.

---

## 3. Architecture Viewpoints

### 3.1 Logical Viewpoint
Focuses on the functional decomposition. The system is designed using the **Linker Pattern** and **Registry-based Opcodes** to isolate physical data from logical application symbols.

### 3.2 Process Viewpoint
Describes runtime behavior, specifically the multi-stage pipeline: **Extraction -> Validation -> Resolution -> Linking**. It emphasizes the usage of **Stateful Readers** for safe binary navigation.

### 3.3 Implementation Viewpoint
Focuses on the organization of software modules. The project follows a strictly **Layered & Fragmented Architecture** to prevent "fat files" and ensure high maintainability.

---

## 4. Architecture Views

### 4.1 Logical View
- **`ValidationRule` Trait**: A rule-based engine for pluggable file integrity checks.
- **`DexResolver` Trait**: Dependency Injection interface for zero-copy symbol resolution.
- **`OpcodeTable`**: A modular, data-driven reference for 100% of Dalvik opcodes (0x00 - 0xFF).
- **`DexMetadata`**: A unified registry for all DEX cross-reference tables.

### 4.2 Process View (Data Pipeline)
1. **Stage 1: Atomic Extraction**: Raw binary data is extracted into physical models (`RawModels`) using the `DexReader`.
2. **Stage 2: Rule-Based Validation**: Sequential execution of `Magic`, `Checksum`, `Offsets`, and `MapList` rules.
3. **Stage 3: Value Resolution**: Zero-copy resolution of string and type descriptors.
4. **Stage 4: Parallel Linking & Assembly**: Orchestrating the `DexLinker` to build high-level logical objects (Classes, Methods, Annotations) across multiple threads.

### 4.3 Implementation View (Module Structure)
- **`readers/`**: Low-level binary stream management (LEB128, Endianness).
- **`parsers/`**: Atomic units (Header, Strings, Annotations, Code, etc.) with internal sub-modules to prevent file bloating.
- **`linker/`**: The central intelligence hub for connecting raw indices into logical relations.
- **`display/`**: Strategy-based output formatters (Standard Text and Pretty JSON).
- **`constants/`**: Centralized, non-hardcoded configuration and DEX specifications.

### 4.4 Deployment View
- **APK/Multidex Support**: Integrated `ApkParser` for automatic ZIP extraction and multi-DEX orchestration.
- **CI/CD Pipeline**: Automated testing, linting, and cross-platform binary releases (Linux, macOS, Windows).

---

## 5. Architecture Rationale
- **Rust Language**: Chosen for fearless concurrency and memory safety without a garbage collector.
- **Rayon**: Selected for high-level data parallelism that outperforms legacy single-core tools.
- **Atomic Modularity**: Adopted to ensure that individual parser units can be updated or tested in isolation.
- **No-Hardcode Policy**: Implemented to centralize DEX specifications and facilitate support for newer Android versions.

---

## 6. Consistency and Completeness
- All logical traits are mapped to granular implementation files.
- The architecture ensures 100% opcode coverage and strict error handling via `Result<T, DexError>`.
- Compliant with ISO 42010 by addressing stakeholders through documented architectural viewpoints.
