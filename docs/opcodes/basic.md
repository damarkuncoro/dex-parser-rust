# Basic & System Opcodes

| Opcode | Name | Description |
|--------|------|-------------|
| 0x00 | OP_NOP | No operation. Waste cycles. |
| 0xec | OP_BREAKPOINT | Development breakpoint (Internal). |
| 0xed | OP_THROW_VERIFICATION_ERROR | Throw error during verification (Internal). |
| 0xee | OP_EXECUTE_INLINE | Execute inline method (Internal). |
| 0xef | OP_EXECUTE_INLINE_RANGE | Execute inline method range (Internal). |

## Details

### nop
Waste cycles. Data-bearing pseudo-instructions are tagged with this opcode, in which case the high-order byte of the opcode unit indicates the nature of the data.

### internal opcodes
- `breakpoint`: Used by debuggers.
- `throw-verification-error`: Thrown when the verifier finds an issue.
- `execute-inline`: Internal optimization for calling certain system methods.
