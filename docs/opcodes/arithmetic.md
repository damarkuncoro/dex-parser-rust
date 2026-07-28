# Arithmetic and Logic Opcodes

Mathematical, bitwise, and comparison operations.

| Opcode | Name | Description |
|--------|------|-------------|
| **Comparisons** | | |
| 0x2d | OP_CMPL_FLOAT | Compare float (lt bias). |
| 0x2e | OP_CMPG_FLOAT | Compare float (gt bias). |
| 0x2f | OP_CMPL_DOUBLE | Compare double (lt bias). |
| 0x30 | OP_CMPG_DOUBLE | Compare double (gt bias). |
| 0x31 | OP_CMP_LONG | Compare long. |
| **Unary Operations** | | |
| 0x7b | OP_NEG_INT | Negate int. |
| 0x7c | OP_NOT_INT | Bitwise NOT int. |
| 0x7d | OP_NEG_LONG | Negate long. |
| 0x7e | OP_NOT_LONG | Bitwise NOT long. |
| 0x7f | OP_NEG_FLOAT | Negate float. |
| 0x80 | OP_NEG_DOUBLE | Negate double. |
| **Binary Operations** | | |
| 0x90-0xaf | OP_ADD/SUB/MUL... | Standard binary operations. |
| 0xb0-0xcf | ..._2ADDR | 2-register variants (vA = vA op vB). |
| 0xd0-0xd7 | ..._LIT16 | Operations with 16-bit literal constant. |
| 0xd8-0xe2 | ..._LIT8 | Operations with 8-bit literal constant. |

## Details

### Comparisons (cmp*)
Returns `0` if `b == c`, `1` if `b > c`, or `-1` if `b < c`.
Floating point "bias" indicates how `NaN` is treated: `gt bias` returns `1`, `lt bias` returns `-1`.

### Binary Operations (binop)
Standard operations: `add`, `sub`, `mul`, `div`, `rem`, `and`, `or`, `xor`, `shl`, `shr`, `ushr`.
Available for `int`, `long`, `float`, and `double`.
Note: `long` shift operations take a single register for the shift count.
