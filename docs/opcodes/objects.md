# Object and Array Opcodes

Operations on objects, arrays, and monitors.

| Opcode | Name | Description |
|--------|------|-------------|
| 0x1d | OP_MONITOR_ENTER | Acquire the monitor for a given object. |
| 0x1e | OP_MONITOR_EXIT | Release the monitor for a given object. |
| 0x1f | OP_CHECK_CAST | Throw ClassCastException if vAA cannot be cast to type@BBBB. |
| 0x20 | OP_INSTANCE_OF | Store 1 in vA if vB is an instance of type@CCCC, else 0. |
| 0x21 | OP_ARRAY_LENGTH | Store the length of array vB in vA. |
| 0x22 | OP_NEW_INSTANCE | Instantiate a new object of type@BBBB. |
| 0x23 | OP_NEW_ARRAY | Instantiate a new array of type@CCCC. |
| 0x24 | OP_FILLED_NEW_ARRAY | Instantiate a new array and fill it with registers. |
| 0x25 | OP_FILLED_NEW_ARRAY_RANGE | Instantiate a new array and fill it with a range of registers. |
| 0x26 | OP_FILL_ARRAY_DATA | Fill an array with a payload from the data section. (See [Pseudo-Instructions](./pseudo.md)) |
| **Array Access (AGET)** | | |
| 0x44 | OP_AGET | Array get. |
| 0x45 | OP_AGET_WIDE | Array get (wide). |
| 0x46 | OP_AGET_OBJECT | Array get (object). |
| 0x47 | OP_AGET_BOOLEAN | Array get (boolean). |
| 0x48 | OP_AGET_BYTE | Array get (byte). |
| 0x49 | OP_AGET_CHAR | Array get (char). |
| 0x4a | OP_AGET_SHORT | Array get (short). |
| **Array Access (APUT)** | | |
| 0x4b | OP_APUT | Array put. |
| 0x4c | OP_APUT_WIDE | Array put (wide). |
| 0x4d | OP_APUT_OBJECT | Array put (object). |
| 0x4e | OP_APUT_BOOLEAN | Array put (boolean). |
| 0x4f | OP_APUT_BYTE | Array put (byte). |
| 0x50 | OP_APUT_CHAR | Array put (char). |
| 0x51 | OP_APUT_SHORT | Array put (short). |
