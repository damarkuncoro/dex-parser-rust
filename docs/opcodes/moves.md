# Move Opcodes

Transfer data between registers.

| Opcode | Name | Description |
|--------|------|-------------|
| 0x01 | OP_MOVE | Move the contents of one non-object register to another. |
| 0x02 | OP_MOVE_FROM16 | Move register contents (16-bit source). |
| 0x03 | OP_MOVE_16 | Move register contents (16-bit dest/source). |
| 0x04 | OP_MOVE_WIDE | Move the contents of one pair of non-object registers to another. |
| 0x05 | OP_MOVE_WIDE_FROM16 | Move register pair contents (16-bit source). |
| 0x06 | OP_MOVE_WIDE_16 | Move register pair contents (16-bit dest/source). |
| 0x07 | OP_MOVE_OBJECT | Move the contents of one object register to another. |
| 0x08 | OP_MOVE_OBJECT_FROM16 | Move object register contents (16-bit source). |
| 0x09 | OP_MOVE_OBJECT_16 | Move object register contents (16-bit dest/source). |
| 0x0a | OP_MOVE_RESULT | Move the result of the previously-called method into a register. |
| 0x0b | OP_MOVE_RESULT_WIDE | Move the wide result of the previously-called method into a register. |
| 0x0c | OP_MOVE_RESULT_OBJECT | Move the object result of the previously-called method into a register. |
| 0x0d | OP_MOVE_EXCEPTION | Move the exception object from the current method's exception register. |

## Details

### move-result*
These must be done as the instruction immediately after an `invoke-kind` whose result is not to be ignored; anywhere else is invalid.

### move-exception
This must be the first instruction of any exception handler whose caught exception is not to be ignored.
