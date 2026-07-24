# Software Architecture Description (SAD) - ISO/IEC/IEEE 42010:2011 Compliant

## 1. Introduction
This document provides a formal architecture description for `dex-parser-rust`, a high-performance, memory-safe, and modular Android DEX/APK parsing engine. It is structured according to the ISO/IEC/IEEE 42010:2011 standard.

---

## 2. The Three Pillars of dex-parser-rust
The entire design of the system is built upon three foundational pillars:

### I. High Performance (Parallelism & WASM)
Leveraging modern multi-core hardware via **Rayon** for desktop and high-speed **WebAssembly** for browser environments. The engine parallelizes class processing, high-level linking, and bytecode disassembly.

### II. Safety & Efficiency (Zero-Copy & Resiliency)
Utilizing Rust's ownership model to implement **Zero-Copy parsing**. Data is referenced directly from the original buffer. The system includes a **Resilient String Resolver** to handle Android's *Modified UTF-8* (MUTF-8) without crashing.

### III. Radical Modularity (Atomic Parsing & Analysis Layer)
Strict separation between **Physical Binary Extraction** (Atomic Parsers), **Logical Symbol Resolution** (Linker Engine), and **Intelligence Layer** (Analysis Engine).

---

## 3. Architecture Viewpoints

### 3.1 Logical Viewpoint
Focuses on the functional decomposition. Designed using the **Linker Pattern** and **Registry-based Opcodes** to isolate physical data from logical application symbols.

### 3.2 Process Viewpoint
Describes runtime behavior, specifically the multi-stage pipeline: **Extraction -> Validation -> Resolution -> Linking -> Analysis**. It supports **Asynchronous execution** via Web Workers in the browser.

### 3.3 Implementation Viewpoint
Focuses on the organization of software modules. Includes **Hybrid Compilation** (Native/WASM), **Auto-Versioning** via build scripts, and **Stateful Session Management** in Rust memory.

---

## 4. Architecture Views

### 4.1 Logical View
- **`ValidationRule` Trait**: A rule-based engine for pluggable file integrity checks.
- **`DexResolver` Trait**: Dependency Injection interface for zero-copy symbol resolution.
- **`OpcodeTable`**: A modular reference for 100% of Dalvik opcodes (0x00 - 0xFF).
- **`ApkContext`**: A global authority for Cross-DEX linking in Multidex environments.
- **`AnalysisEngine`**: Modular units for XREF (Cross-Reference) and CFG (Control Flow Graph).

### 4.2 Process View (Data Pipeline)
1. **Stage 1: Atomic Extraction**: Raw binary data is extracted into `RawModels`.
2. **Stage 2: Rule-Based Validation**: Sequential execution of integrity rules.
3. **Stage 3: Value Resolution**: Zero-copy resolution with MUTF-8 resiliency.
4. **Stage 4: Parallel Linking & Assembly**: Orchestrating the `DexLinker` to build a unified `Apk` model.
5. **Stage 5: Intelligence Analysis**: Generating XREFs and Basic Blocks for the CFG.

### 4.3 Implementation View (Module Structure)
- **`readers/`**: Low-level binary stream management (LEB128, Endianness).
- **`parsers/`**: Atomic units with internal sub-modules (including modern 038+ tables).
- **`analysis/`**: Intelligence layer for semantic conclusions (XREF, CFG).
- **`wasm.rs`**: Stateful bridge between Rust memory and JavaScript.
- **`ffi.rs`**: C-compatible bridge for external language support (Python, C++).

### 4.4 Deployment View
- **Polyglot & Multi-Platform**: Available as a standalone CLI, Rust library, Shared Object library (`cdylib`), and WebAssembly module.
- **Web Sandbox**: A zero-install browser environment using Web Workers for non-blocking analysis of large APKs.
- **CI/CD Pipeline**: Automated multi-platform releases via GitHub Actions.

---

## 5. Architecture Rationale
- **Rust Language**: Chosen for memory safety and zero-cost abstractions.
- **Stateful WASM**: Implemented to avoid the massive overhead of serializing large APK data into JavaScript objects.
- **Hybrid Concurrent Model**: Uses Rayon for CPU-bound tasks on Desktop and Web Workers for thread-isolation in Browsers.
- **Separation of Concerns**: Parser focuses on data fidelity; Analysis Engine focuses on semantic intelligence.

---

## 6. Consistency and Completeness
- All logical traits are mapped to granular implementation files.
- The architecture ensures 100% opcode coverage and strict error handling via `Result<T, DexError>`.
- Compliant with ISO 42010.
