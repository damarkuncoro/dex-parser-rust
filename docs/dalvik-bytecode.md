# Dalvik Bytecode

This document describes the general design of the Dalvik bytecode and links to the detailed opcode references.

## General Design

- **Machine Model**: The Dalvik VM is register-based. Frames are fixed in size upon creation. Each frame consists of a particular number of registers as well as adjunct data (PC, `.dex` file reference, etc.).
- **Registers**:
    - Registers are 32 bits wide when used for bit values (integers, floats).
    - Adjacent register pairs are used for 64-bit values (longs, doubles). There is no alignment requirement.
    - For object references, registers hold exactly one reference.
    - Bitwise representation: `(Object) null == (int) 0`.
- **Arguments**: The *N* arguments to a method land in the last *N* registers of the invocation frame. Wide arguments consume two registers. Instance methods receive `this` as their first argument.
- **Instruction Stream**: 16-bit unsigned quantities.
- **Constant Pools**: Separately enumerated and indexed pools for strings, types, fields, and methods.
- **Pseudo-instructions**: Used to hold variable-length data payloads (e.g., `fill-array-data`). These must be 4-byte aligned and typically appear at the end of methods.
- **Mnemonics and Syntax**:
    - Dest-then-source ordering for arguments.
    - Suffixes indicate types: `-wide` (64-bit), `-boolean`, `-byte`, `-char`, `-short`, `-int`, `-long`, `-float`, `-double`, `-object`, `-string`, `-class`, `-void`.
    - Opcode suffixes (e.g., `/from16`) distinguish variants with different layouts.

## Opcode Categories

For detailed opcode references, see the following categories:

1. [**Basic & System**](./opcodes/basic.md)
2. [**Move Operations**](./opcodes/moves.md)
3. [**Constants**](./opcodes/consts.md)
4. [**Control Flow**](./opcodes/flow.md)
5. [**Objects & Arrays**](./opcodes/objects.md)
6. [**Method Invocation**](./opcodes/invokes.md)
7. [**Field Access**](./opcodes/fields.md)
8. [**Arithmetic & Logic**](./opcodes/arithmetic.md)
9. [**Type Conversion**](./opcodes/conversion.md)
10. [**Pseudo-Instructions**](./opcodes/pseudo.md)

## Reference
- [Instruction Formats](./instruction-formats.md)
- [Official Android Dalvik Bytecode Spec](https://source.android.com/devices/tech/dalvik/dalvik-bytecode)
