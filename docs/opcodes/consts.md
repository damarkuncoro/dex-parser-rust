# Constant Opcodes

Load literal values and constant pool references into registers.

| Opcode | Name | Description |
|--------|------|-------------|
| 0x12 | OP_CONST_4 | Load a 4-bit constant (sign-extended to 32 bits). |
| 0x13 | OP_CONST_16 | Load a 16-bit constant (sign-extended to 32 bits). |
| 0x14 | OP_CONST | Load a 32-bit constant. |
| 0x15 | OP_CONST_HIGH16 | Load a 16-bit constant into the high part of a register. |
| 0x16 | OP_CONST_WIDE_16 | Load a 16-bit constant (sign-extended to 64 bits). |
| 0x17 | OP_CONST_WIDE_32 | Load a 32-bit constant (sign-extended to 64 bits). |
| 0x18 | OP_CONST_WIDE | Load a 64-bit constant. |
| 0x19 | OP_CONST_WIDE_HIGH16 | Load a 16-bit constant into the high part of a wide register. |
| 0x1a | OP_CONST_STRING | Load a reference to the string specified by the index. |
| 0x1b | OP_CONST_STRING_JUMBO | Load a string constant with a 32-bit index. |
| 0x1c | OP_CONST_CLASS | Load a reference to the class specified by the index. |
| 0xfe | OP_CONST_METHOD_HANDLE | Load a method handle constant (API 28+). |
| 0xff | OP_CONST_METHOD_TYPE | Load a method type (prototype) constant (API 28+). |
