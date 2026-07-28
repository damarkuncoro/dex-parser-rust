# Dalvik Executable (.dex) Format

This directory contains the documentation for the DEX file format, split into logical sections.

## Sections

1. [**Introduction & Layout**](./format/intro.md) - Guide to types, LEB128, and general file layout.
2. [**Definitions**](./format/definitions.md) - Magic numbers, endianness, access flags, and MUTF-8.
3. [**Items & IDs**](./format/items.md) - Detailed structure of headers and ID tables (String, Type, Proto, Field, Method).
4. [**Classes & Code**](./format/classes.md) - Class definitions, class data, and bytecode items.
5. [**Annotations**](./format/annotations.md) - Directory, set, and item structures for annotations.
6. [**Encoded Values**](./format/values.md) - Format for hierarchical data, call sites, and method handles.
7. [**System Annotations**](./format/system-annotations.md) - Special Dalvik system-level annotations.

## Reference
- [Official Android DEX Format Spec](https://source.android.com/docs/core/runtime/dex-format)
