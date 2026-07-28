# Control Flow Opcodes

Jumps, branches, returns, and switches.

| Opcode | Name | Description |
|--------|------|-------------|
| 0x0e | OP_RETURN_VOID | Return from a void method. |
| 0x0f | OP_RETURN | Return from a non-void method. |
| 0x10 | OP_RETURN_WIDE | Return from a wide non-void method. |
| 0x11 | OP_RETURN_OBJECT | Return from an object non-void method. |
| 0x27 | OP_THROW | Throw an exception. |
| 0x28 | OP_GOTO | Unconditional 8-bit jump. |
| 0x29 | OP_GOTO_16 | Unconditional 16-bit jump. |
| 0x2a | OP_GOTO_32 | Unconditional 32-bit jump. |
| 0x2b | OP_PACKED_SWITCH | Multi-way branch using a packed table. (See [Pseudo-Instructions](./pseudo.md)) |
| 0x2c | OP_SPARSE_SWITCH | Multi-way branch using a sparse table. (See [Pseudo-Instructions](./pseudo.md)) |
| **Conditionals** | | |
| 0x32 | OP_IF_EQ | Jump if vA == vB. |
| 0x33 | OP_IF_NE | Jump if vA != vB. |
| 0x34 | OP_IF_LT | Jump if vA < vB. |
| 0x35 | OP_IF_GE | Jump if vA >= vB. |
| 0x36 | OP_IF_GT | Jump if vA > vB. |
| 0x37 | OP_IF_LE | Jump if vA <= vB. |
| 0x38 | OP_IF_EQZ | Jump if vA == 0. |
| 0x39 | OP_IF_NEZ | Jump if vA != 0. |
| 0x3a | OP_IF_LTZ | Jump if vA < 0. |
| 0x3b | OP_IF_GEZ | Jump if vA >= 0. |
| 0x3c | OP_IF_GTZ | Jump if vA > 0. |
| 0x3d | OP_IF_LEZ | Jump if vA <= 0. |
| **Internal** | | |
| 0x73 | OP_RETURN_VOID_NO_BARRIER | Internal return-void without memory barrier. |

## Details

### packed-switch & sparse-switch
These instructions reference a separate data payload that contains the targets. The payload must be 4-byte aligned.

### if-*
Branch to the given destination if the comparison is true. The offset is signed 16-bit, relative to the current instruction.
