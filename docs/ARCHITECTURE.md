# Software Architecture Description (SAD) - ISO/IEC/IEEE 42010:2011 Compliant

## 1. Introduction
This document provides a formal architecture description for `dex-parser-rust`, a high-performance, memory-safe, and modular Android DEX/APK parsing engine. It is structured according to the ISO/IEC/IEEE 42010:2011 standard.

---

## 2. The Four Pillars of dex-parser-rust
The design has evolved to support forensic capabilities, adding a fourth foundational pillar:

### I. High Performance (Parallelism & SIMD)
Leveraging modern multi-core hardware via **Rayon** and high-speed **WebAssembly**. Performance is further enhanced via **SIMD-accelerated** string scanning (using `memchr`) and **Chunked-Summing** Adler-32 checksums to minimize CPU cycles.

### II. Safety & Efficiency (Zero-Copy & Resiliency)
Utilizing Rust's ownership model for **Zero-Copy parsing**. Data is referenced directly from the original buffer. A **Byte Tracking Engine** ensures 100% of the binary stream is accounted for, leaving no data hidden.

### III. Modular SSOT (Single Source of Truth)
A centralized **Intelligence Layer** (`AnalysisReport`) serves as the SSOT for all security findings. This modular design separates data models, parser logic, and forensic conclusions.

### IV. Forensic Intelligence (Entropy & Scanning)
Integrating automated intelligence to detect **Packers/Obfuscation** via Shannon Entropy analysis and **Sensitive Data Leakage** via high-performance regex scanning.

---

## 3. Architecture Viewpoints

### 3.1 Logical Viewpoint
Functional decomposition using the **Linker Pattern** and **Modular Opcodes**. Registry-based opcodes are pre-compiled for O(1) lookup during disassembly.

### 3.2 Process Viewpoint
The pipeline is now a 6-stage process: **Extraction -> Validation -> Resolution -> Linking -> Analysis -> Intelligence Output**.

### 3.3 Implementation Viewpoint
Organization of software modules. Includes **Hybrid Compilation**, **Parking-Lot Mutexes** for low-contention parallelism, and **Modular Constant Registry**.

---

## 4. Architecture Views

### 4.1 Logical View
- **`ValidationRule`**: Rule-based engine for pluggable file integrity checks.
- **`ByteTracker`**: A bitset-based monitor that maps binary usage in real-time.
- **`DexLinker`**: Central authority for logical symbol binding and proto-resolution.
- **`AnalysisReport`**: The SSOT for security findings (Entropy, IPs, URLs, API Keys).
- **`OpcodeRegistry`**: Pre-compiled Dalvik instruction metadata for maximum speed.

### 4.2 Process View (Data Pipeline)
1. **Stage 1: Atomic Extraction**: Raw binary data is extracted into `RawModels`.
2. **Stage 2: Integrity Validation**: Parallel execution of rules (Magic, Checksum, Offsets).
3. **Stage 3: Value Resolution**: SIMD-accelerated resolution with MUTF-8 resiliency.
4. **Stage 4: Modular Linking**: Symbol binding with automated parameter resolution.
5. **Stage 5: Data Tracking**: Calculating unreferenced "gaps" to find stealth payloads.
6. **Stage 6: Intelligence Analysis**: Calculating Shannon Entropy and scanning for sensitive indicators.

### 4.3 Implementation View (Module Structure)
- **`core/`**: Foundational logic (Constants, Instructions, Models, Linker, Utils).
- **`readers/`**: Optimized binary stream management (LEB128, Fast-Seeks).
- **`parsers/`**: Atomic parsing units orchestrated by a centralized `MapProcessor`.
- **`analysis/`**: Intelligence layer for semantic conclusions (Entropy, Scanner, CFG).
- **`wasm.rs`**: Clean glue layer between Rust core and JS frontend.

### 4.4 Deployment View
- **Multi-Platform Native**: Optimized binaries for Linux, macOS, and Windows.
- **Web Sandbox**: Zero-install environment with **Tiered Forensic Alerting** (Red/Yellow/Green).
- **Automated CD**: Multi-arch build pipeline via GitHub Actions.

---

## 5. Architecture Rationale
- **Parking Lot vs Std**: `parking_lot::Mutex` chosen for performance in high-concurrency class linking.
- **Shannon Entropy**: Implemented to provide a data-agnostic method for detecting encrypted payloads.
- **Modular Constants**: Decoupled constants into category-specific files to avoid "God Object" anti-patterns.
- **Zero-Copy Regex**: Optimized for `&[u8]` to eliminate string allocation overhead during mass metadata scanning.

---

## 6. Consistency and Completeness
- All security findings are aggregated in the `AnalysisReport` for cross-platform consistency.
- Performance optimizations (SIMD/Chunking) are implemented at the lowest utility level to benefit all stages.
- Compliant with ISO 42010.
