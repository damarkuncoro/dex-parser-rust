# Software Architecture Description (SAD) - ISO/IEC/IEEE 42010:2011 Compliant

## 1. Introduction
This document provides a formal architecture description for `dex-parser-rust`, a high-performance Android DEX file parsing engine. It is structured according to the ISO/IEC/IEEE 42010:2011 standard for architecture descriptions.

## 2. Stakeholders and Concerns
| Stakeholder | Concerns |
| :--- | :--- |
| **Security Researchers** | Memory safety, accuracy of disassembled bytecode, ability to handle malformed files. |
| **Software Developers** | Reusability as a library, clear API documentation, extensibility for new DEX versions. |
| **DevOps Engineers** | Build performance, cross-platform compatibility, CI/CD automation. |
| **System Architects** | Modularity, performance scalability on multi-core systems, technical debt management. |

## 3. Architecture Viewpoints

### 3.1 Logical Viewpoint
Focuses on the functional requirements—what the system should do. It describes the design's object-oriented decomposition and module boundaries.

### 3.2 Process Viewpoint
Focuses on the runtime behavior, specifically data flow and concurrency management (Parallel Parsing).

### 3.3 Implementation (Development) Viewpoint
Focuses on the organization of actual software modules in the development environment, including layers and dependencies.

### 3.4 Deployment Viewpoint
Focuses on the distribution of the system across platforms and the automated delivery pipeline.

---

## 4. Architecture Views

### 4.1 Logical View
The system utilizes **Dependency Injection (DI)** via Rust Traits to ensure loose coupling between components.
- **`DexResolver` Trait**: Abstract interface for resolving string, type, method, and field references.
- **`DexPrinter` Trait**: Strategy pattern for output formatting.
- **Model Decomposition**: Strong typing for DEX structures (Header, Class, Method, Field) with Serde support.

### 4.2 Process View (Data Flow & Concurrency)
1. **Header Validation**: Sequential parsing of the file header and endianness detection.
2. **Metadata Extraction**: Sequential parsing of cross-reference tables (Strings, Types, Protos, Fields, Methods).
3. **Parallel Class Processing**: Utilizing **Rayon's work-stealing algorithm** to parallelize the disassembly and parsing of class definitions across all available CPU cores.
4. **Symbol Resolution**: Concurrent lookups via the shared `DexContext`.

### 4.3 Implementation View
The project follows a **Layered Architecture**:
- **Presentation Layer**: `cli.rs` and `display/` (Handles UI and formatting).
- **Core Logic Layer**: `parsers/` and `instructions/` (Main engine logic).
- **Domain/Data Layer**: `models/` and `context.rs` (Data structures and state).
- **Support Layer**: `utils/`, `error.rs`, and `constants.rs`.

### 4.4 Deployment View
- **Build System**: Cargo (Rust's package manager).
- **Target Platforms**: Linux (x86_64), macOS (Intel/ARM), Windows (MSVC).
- **CI/CD**: GitHub Actions automated pipeline for linting (Clippy), formatting (Fmt), and multi-platform releases.

---

## 5. Architecture Rationale
- **Language Selection (Rust)**: Chosen for memory safety (eliminating common C/C++ vulnerabilities like buffer overflows) and zero-cost abstractions.
- **Parallelization (Rayon)**: Selected to exceed the performance of the single-threaded `dexdump` tool on multi-core hardware.
- **Trait-based DI**: Implemented to facilitate unit testing with mock objects and enable library reusability.
- **Serde**: Integrated for high-performance serialization, allowing the engine to easily provide JSON output for external tool chains.

---

## 6. Consistency and Completeness
- All logical components (Traits) are mapped directly to implementation files in `src/`.
- Concurrency models (Rayon) are constrained by Rust's `Send + Sync` traits to ensure thread safety.
- The architecture supports full ISO compliance by addressing all stakeholder concerns through specific viewpoints.
