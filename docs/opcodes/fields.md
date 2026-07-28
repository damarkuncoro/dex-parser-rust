# Field Access Opcodes

Reading and writing instance and static fields.

| Opcode | Name | Description |
|--------|------|-------------|
| **Instance Fields (IGET)** | | |
| 0x52 | OP_IGET | Instance field get. |
| 0x53 | OP_IGET_WIDE | Instance field get (wide). |
| 0x54 | OP_IGET_OBJECT | Instance field get (object). |
| 0x55 | OP_IGET_BOOLEAN | Instance field get (boolean). |
| 0x56 | OP_IGET_BYTE | Instance field get (byte). |
| 0x57 | OP_IGET_CHAR | Instance field get (char). |
| 0x58 | OP_IGET_SHORT | Instance field get (short). |
| **Instance Fields (IPUT)** | | |
| 0x59 | OP_IPUT | Instance field put. |
| 0x5a | OP_IPUT_WIDE | Instance field put (wide). |
| 0x5b | OP_IPUT_OBJECT | Instance field put (object). |
| 0x5c | OP_IPUT_BOOLEAN | Instance field put (boolean). |
| 0x5d | OP_IPUT_BYTE | Instance field put (byte). |
| 0x5e | OP_IPUT_CHAR | Instance field put (char). |
| 0x5f | OP_IPUT_SHORT | Instance field put (short). |
| **Static Fields (SGET)** | | |
| 0x60 | OP_SGET | Static field get. |
| 0x61 | OP_SGET_WIDE | Static field get (wide). |
| 0x62 | OP_SGET_OBJECT | Static field get (object). |
| 0x63 | OP_SGET_BOOLEAN | Static field get (boolean). |
| 0x64 | OP_SGET_BYTE | Static field get (byte). |
| 0x65 | OP_SGET_CHAR | Static field get (char). |
| 0x66 | OP_SGET_SHORT | Static field get (short). |
| **Static Fields (SPUT)** | | |
| 0x67 | OP_SPUT | Static field put. |
| 0x68 | OP_SPUT_WIDE | Static field put (wide). |
| 0x69 | OP_SPUT_OBJECT | Static field put (object). |
| 0x6a | OP_SPUT_BOOLEAN | Static field put (boolean). |
| 0x6b | OP_SPUT_BYTE | Static field put (byte). |
| 0x6c | OP_SPUT_CHAR | Static field put (char). |
| 0x6d | OP_SPUT_SHORT | Static field put (short). |

## Advanced & Optimized

### Volatile Fields

| Opcode | Name | Description |
|--------|------|-------------|
| 0xe3 | OP_IGET_VOLATILE | Instance field get (volatile). |
| 0xe4 | OP_IPUT_VOLATILE | Instance field put (volatile). |
| 0xe5 | OP_SGET_VOLATILE | Static field get (volatile). |
| 0xe6 | OP_SPUT_VOLATILE | Static field put (volatile). |
| 0xe7 | OP_IGET_OBJECT_VOLATILE | Instance field get (object, volatile). |
| 0xe8 | OP_IGET_WIDE_VOLATILE | Instance field get (wide, volatile). |
| 0xe9 | OP_IPUT_WIDE_VOLATILE | Instance field put (wide, volatile). |
| 0xea | OP_SGET_WIDE_VOLATILE | Static field get (wide, volatile). |
| 0xeb | OP_SPUT_WIDE_VOLATILE | Static field put (wide, volatile). |
| 0xf0 | OP_IPUT_OBJECT_VOLATILE | Instance field put (object, volatile). |
| 0xf1 | OP_SGET_OBJECT_VOLATILE | Static field get (object, volatile). |

### Quickened Fields
These are used by the runtime after optimization.

| Opcode | Name | Description |
|--------|------|-------------|
| 0xf2 | OP_IGET_QUICK | Optimized instance field get. |
| 0xf3 | OP_IGET_WIDE_QUICK | Optimized instance field get (wide). |
| 0xf4 | OP_IGET_OBJECT_QUICK| Optimized instance field get (object). |
| 0xf5 | OP_IPUT_QUICK | Optimized instance field put. |
| 0xf6 | OP_IPUT_WIDE_QUICK | Optimized instance field put (wide). |
| 0xf7 | OP_IPUT_OBJECT_QUICK| Optimized instance field put (object). |
