# Changelog

All notable changes to this project will be documented in this file.

## [1.5.0] - 2024-06-16

### Added
- **Deep Modular Parser Architecture**: Categorized DEX parsers into `core`, `identifiers`, and `definitions` for better maintainability and encapsulation.
- **Resource ID Resolution**: Implemented `ArscParser` and `ResourceVisitor` to map binary Resource IDs (0x7f...) to human-readable names from `resources.arsc`.
- **Call Graph Visualization**: Added `--call-graph` CLI option to export global method-to-method relationships in Graphviz DOT format.
- **Manifest Forensic Analysis**: New `ManifestAnalyzer` to detect dangerous permission combinations, persistence via boot-completed receivers, and stealth components.
- **Parallel Visitor Pipeline**: All analysis passes (Stats, XREF, Behavior, Obfuscation, Resources) now run in a single optimized parallel walk.

### Fixed
- **AXML Decoding Robustness**: Enhanced handling of UTF-8 and UTF-16 string pools in Android Binary XML, supporting complex length encodings.
- **Namespace Resolution**: Consistent use of `Arc<AnalysisConfig>` across parallel visitors to ensure thread-safety and eliminate lifetime issues.

### Changed
- **CLI Enhancements**: Added real-time progress feedback on `stderr` and support for full APK intelligence summaries.
- **Unified Global Intelligence**: Consolidated cross-DEX findings and resolved resources into a single APK-level report.

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
