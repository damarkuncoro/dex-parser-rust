# Changelog

All notable changes to this project will be documented in this file.

## [1.4.0] - 2024-06-15

### Added
- **Modular Analysis Architecture**: Introduced `InstructionVisitor` and `AnalysisPipeline` allowing single-pass parallel analysis for XREF, Statistics, Behavior, and Obfuscation.
- **Android Binary XML (AXML) Parser**: Robust initial implementation for parsing `AndroidManifest.xml`, extracting package name, permissions, and components.
- **Forensic Obfuscation Detection**: New `ObfuscationVisitor` to detect suspiciously long names and non-ASCII identifiers in classes and methods.
- **Progress Feedback**: Real-time progress indicators on stderr during heavy APK/DEX analysis.
- **Structured Instruction IR**: Instructions now store parsed register indices and absolute target offsets, enabling faster and more accurate Control Flow Graph (CFG) building.

### Fixed
- **Integer Overflow**: Fixed a critical panic on very large DEX files during register extraction.
- **AXML String Decoding**: Fixed inaccuracies in UTF-8/UTF-16 string pool decoding for Android Binary XML.
- **Resilient APK Parsing**: APK extraction no longer halts on a single corrupted DEX file.
- **Memory Safety**: Added bounds checking in manual binary parsing sections to prevent panics on malformed files.

### Changed
- **Documentation**: Converted legacy HTML documentation into an organized Markdown structure under `docs/`.
- **Parallel Optimization**: Consolidated multiple analysis passes into a single parallel walk using `rayon`, significantly reducing CPU cycles for large APKs.
- **Opcode Coverage**: Increased opcode identification coverage to >96% (including volatile and internal opcodes).
