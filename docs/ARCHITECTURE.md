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
Focuses on the functional requirements and the object-oriented/module decomposition. The system is designed using **Atomic Parsing & Linking** patterns to isolate physical binary extraction from logical symbol resolution.

### 3.2 Process Viewpoint
Focuses on the runtime behavior, specifically data flow and concurrency management. It highlights the use of **Zero-Copy** techniques and **Parallel Class Processing**.

### 3.3 Implementation (Development) Viewpoint
Focuses on the organization of actual software modules. The project follows a strictly **Layered Architecture** with high internal cohesion and low coupling between parsers.

### 3.4 Deployment Viewpoint
Focuses on the distribution across platforms and the automated release pipeline via GitHub Actions.

---

## 4. Architecture Views

### 4.1 Logical View
The system utilizes **Dependency Injection (DI)** via Rust Traits and a **Rule-Based Validation Engine**.
- **`ValidationRule` Trait**: Allows for a modular, extensible validation system where rules (Magic, Checksum, Offsets) can be added/removed without changing the orchestrator.
- **`DexResolver` Trait**: Abstract interface for resolving string, type, method, and field references, enabling zero-copy symbol resolution.
- **`DexPrinter` Trait**: Strategy pattern for output formatting (Text, JSON).
- **Atomic Parsers**: Each parser module (Header, String, Type, etc.) is independent and only extracts raw structures.

### 4.2 Process View (Data Flow & Concurrency)
1. **Atomic Extraction (Stage 1)**: Using the stateful `DexReader` to extract raw binary structures into `RawModels`.
2. **Rule-Based Validation**: Executing the `DexValidator` engine to ensure file integrity.
3. **Value Resolution (Stage 2)**: Converting binary offsets into string references using **Zero-Copy** lifetimes (`&'a str`).
4. **Logical Linking (Stage 3)**: The `DexLinker` engine connects raw indices into high-level logical objects (e.g., connecting a Method index to its Class and Signature).
5. **Parallel High-Level Assembly (Stage 4)**: Utilizing **Rayon** to parallelize class data parsing and disassembly across multiple CPU cores.

### 4.3 Implementation View
The implementation is highly modularized to avoid "fat files" and ensure scalability:
- **`readers/`**: Encapsulates low-level binary reading and LEB128 decoding.
- **`parsers/`**: Contains independent, atomic parsing units for each DEX section.
- **`linker/`**: Central hub for logical symbol resolution.
- **`models/`**: Separates `Raw` (physical) models from high-level logical models.
- **`display/`**: Separates presentation logic into `text` and `json` sub-modules.

### 4.4 Deployment View
- **Multi-Platform CI/CD**: Fully automated pipeline builds and tests on Linux, macOS (x64/ARM), and Windows.
- **Release Strategy**: Automated binary packaging and GitHub Release generation upon tag creation.

---

## 5. Architecture Rationale
- **Zero-Copy Lifetime Management**: Chosen to maximize performance and minimize heap allocations by referencing the original buffer.
- **Parallelization (Rayon)**: Selected to leverage modern multi-core hardware, making the parser significantly faster than single-threaded legacy tools.
- **Atomic Parsing**: Adopted to ensure that changes in one DEX section's specification do not impact other parser components.
- **Stateful `DexReader`**: Implemented to provide a safe, stateful cursor that automatically handles bounds checking and endianness.

---

## 6. Consistency and Completeness
- All logical components are mapped directly to implementation files in `src/`.
- The architecture ensures that no parser depends on another, maintaining a "shared-nothing" design until the linking stage.
- Compliant with ISO 42010 by addressing all primary stakeholder concerns through multiple architectural views.
