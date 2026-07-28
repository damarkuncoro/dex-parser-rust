# DEX Format: Definitions

## Bitfield, string, and constant definitions

### DEX_FILE_MAGIC
The list of bytes that must appear at the beginning of a `.dex` file.
```
ubyte[8] DEX_FILE_MAGIC = { 0x64 0x65 0x78 0x0a 0x30 0x33 0x39 0x00 }
                        = "dex\n039\0"
```

### ENDIAN_CONSTANT
Indicates the endianness of the file.
```
uint ENDIAN_CONSTANT = 0x12345678;
uint REVERSE_ENDIAN_CONSTANT = 0x78563412;
```

### NO_INDEX
Indicates that an index value is absent.
```
uint NO_INDEX = 0xffffffff;
```

### access_flags definitions
Bitfields indicating accessibility and properties of classes and members.
- `ACC_PUBLIC` (0x1)
- `ACC_PRIVATE` (0x2)
- `ACC_PROTECTED` (0x4)
- `ACC_STATIC` (0x8)
- `ACC_FINAL` (0x10)
- `ACC_INTERFACE` (0x200)
- `ACC_ABSTRACT` (0x400)
- `ACC_CONSTRUCTOR` (0x10000)

### MUTF-8 (Modified UTF-8)
DEX string data is encoded in MUTF-8:
- Only 1, 2, and 3-byte encodings are used.
- Surrogate pairs are used for `U+10000`...`U+10ffff`.
- `U+0000` is encoded as `0xc0 0x80`.
- Null byte `0x00` terminates the string.
