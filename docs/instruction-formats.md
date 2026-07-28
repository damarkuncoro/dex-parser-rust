# Dalvik Executable Instruction Formats

This page lists the instruction formats used by the Dalvik executable (DEX) format and Dalvik bytecode. It is meant to be used in conjunction with the [bytecode reference document](./dalvik-bytecode.md).

## Bitwise descriptions

The first column in the format table lists the bitwise layout of the format. It consists of one or more space-separated "words" each of which describes a 16-bit code unit. Each character in a word represents four bits, read from high bits to low, with vertical bars (`|`) interspersed to aid in reading. Uppercase letters in sequence from `A` are used to indicate fields within the format (which then get defined further by the syntax column). The term `op` is used to indicate the position of an eight-bit opcode within the format. A slashed zero (`Ø`) is used to indicate that all bits must be zero in the indicated position.

For the most part, lettering proceeds from earlier code units to later code units, and low-order to high-order within a code unit. However, there are a few exceptions to this general rule, which are done in order to make the naming of similar-meaning parts be the same across different instruction formats. These cases are noted explicitly in the format descriptions.

For example, the format `B|A|op CCCC` indicates that the format consists of two 16-bit code units. The first word consists of the opcode in the low eight bits and a pair of four-bit values in the high eight bits; and the second word consists of a single 16-bit value.

## Format IDs

The second column in the format table indicates the short identifier for the format, which is used in other documents and in code to identify the format.

Most format IDs consist of three characters, two digits followed by a letter. The first digit indicates the number of 16-bit code units in the format. The second digit indicates the maximum number of registers that the format contains (maximum, since some formats can accommodate a variable number of registers), with the special designation `r` indicating that a range of registers is encoded. The final letter semi-mnemonically indicates the type of any extra data encoded by the format. For example, format `21t` is of length two, contains one register reference, and additionally contains a branch target.

Suggested static linking formats have an additional `s` suffix, making them four characters total. Similarly, suggested "inline" linking formats have an additional `i` suffix. (In this context, inline linking is like static linking, except with more direct ties into a machine's implementation.) Finally, a couple oddball suggested formats (e.g., `20bc`) include two pieces of data which are both represented in its format ID.

The full list of typecode letters are as follows:

| Mnemonic | Bit Sizes | Meaning |
| :--- | :--- | :--- |
| b | 8 | immediate signed **b**yte |
| c | 16, 32 | **c**onstant pool index |
| f | 16 | inter**f**ace constants (only used in statically linked formats) |
| h | 16 | immediate signed **h**at (high-order bits of a 32- or 64-bit value; low-order bits are all `0`) |
| i | 32 | immediate signed **i**nt, or 32-bit float |
| l | 64 | immediate signed **l**ong, or 64-bit double |
| m | 16 | **m**ethod constants (only used in statically linked formats) |
| n | 4 | immediate signed **n**ibble |
| s | 16 | immediate signed **s**hort |
| t | 8, 16, 32 | branch **t**arget |
| x | 0 | no additional data |

## Syntax

The third column of the format table indicates the human-oriented syntax for instructions which use the indicated format. Each instruction starts with the named opcode and is optionally followed by one or more arguments, themselves separated with commas.

Wherever an argument refers to a field from the first column, the letter for that field is indicated in the syntax, repeated once for each four bits of the field. For example, an eight-bit field labeled `BB` in the first column would also be labeled `BB` in the syntax column.

Arguments which name a register have the form `vX`. The prefix `v` was chosen instead of the more common `r` exactly to avoid conflicting with (non-virtual) architectures on which the Dalvik Executable format might be implemented.

Arguments which indicate a literal value have the form `#+X`. Some formats indicate literals that only have non-zero bits in their high-order bits; for these, the zeroes are represented explicitly in the syntax, even though they do not appear in the bitwise representation.

Arguments which indicate a relative instruction address offset have the form `+X`.

Arguments which indicate a literal constant pool index have the form `kind@X`, where `kind` indicates which constant pool is being referred to. Each opcode that uses such a format explicitly allows only one kind of constant. The kinds of constant pool are `string` (string pool index), `type` (type pool index), `field` (field pool index), `meth` (method pool index), and `site` (call site index).

Similar to the representation of constant pool indices, there are also suggested (optional) forms that indicate prelinked offsets or indices. There are two types of suggested prelinked value: vtable offsets (`vtaboff`) and field offsets (`fieldoff`).

In the cases where a format value isn't explicitly part of the syntax but instead picks a variant, each variant is listed with the prefix `[X=N]` (e.g., `[A=2]`) to indicate the correspondence.

## Formats

| Format | ID | Syntax | Notable Opcodes Covered |
| :--- | :--- | :--- | :--- |
| *N/A* | 00x | *N/A* | *pseudo-format used for unused opcodes* |
| ØØ\|*op* | 10x | *op* | |
| B\|A\|*op* | 12x | *op* vA, vB | |
| B\|A\|*op* | 11n | *op* vA, #+B | |
| AA\|*op* | 11x | *op* vAA | |
| AA\|*op* | 10t | *op* +AA | goto |
| ØØ\|*op* AAAA | 20t | *op* +AAAA | goto/16 |
| AA\|*op* BBBB | 20bc | *op* AA, kind@BBBB | *suggested format for verification errors* |
| AA\|*op* BBBB | 22x | *op* vAA, vBBBB | |
| AA\|*op* BBBB | 21t | *op* vAA, +BBBB | |
| AA\|*op* BBBB | 21s | *op* vAA, #+BBBB | |
| AA\|*op* BBBB | 21h | *op* vAA, #+BBBB0000 / #+BBBB000000000000 | |
| AA\|*op* BBBB | 21c | *op* vAA, kind@BBBB | check-cast, const-class, const-string, etc. |
| AA\|*op* CC\|BB | 23x | *op* vAA, vBB, vCC | |
| AA\|*op* CC\|BB | 22b | *op* vAA, vBB, #+CC | |
| B\|A\|*op* CCCC | 22t | *op* vA, vB, +CCCC | |
| B\|A\|*op* CCCC | 22s | *op* vA, vB, #+CCCC | |
| B\|A\|*op* CCCC | 22c | *op* vA, vB, kind@CCCC | instance-of |
| B\|A\|*op* CCCC | 22cs | *op* vA, vB, fieldoff@CCCC | |
| ØØ\|*op* AAAAlo AAAAhi | 30t | *op* +AAAAAAAA | goto/32 |
| ØØ\|*op* AAAA BBBB | 32x | *op* vAAAA, vBBBB | |
| AA\|*op* BBBBlo BBBBhi | 31i | *op* vAA, #+BBBBBBBB | |
| AA\|*op* BBBBlo BBBBhi | 31t | *op* vAA, +BBBBBBBB | |
| AA\|*op* BBBBlo BBBBhi | 31c | *op* vAA, string@BBBBBBBB | const-string/jumbo |
| A\|G\|*op* BBBB F\|E\|D\|C | 35c | *[A=5]* *op* {vC, vD, vE, vF, vG}, kind@BBBB | invoke-kind, filled-new-array |
| A\|G\|*op* BBBB F\|E\|D\|C | 35ms | *[A=5]* *op* {vC, vD, vE, vF, vG}, vtaboff@BBBB | |
| A\|G\|*op* BBBB F\|E\|D\|C | 35mi | *[A=5]* *op* {vC, vD, vE, vF, vG}, inline@BBBB | |
| AA\|*op* BBBB CCCC | 3rc | *op* {vCCCC .. vNNNN}, kind@BBBB | invoke-kind/range, filled-new-array/range |
| AA\|*op* BBBB CCCC | 3rms | *op* {vCCCC .. vNNNN}, vtaboff@BBBB | |
| AA\|*op* BBBB CCCC | 3rmi | *op* {vCCCC .. vNNNN}, inline@BBBB | |
| A\|G\|*op* BBBB F\|E\|D\|C HHHH | 45cc | *[A=5]* *op* {vC, vD, vE, vF, vG}, meth@BBBB, proto@HHHH | invoke-polymorphic |
| AA\|*op* BBBB CCCC HHHH | 4rcc | *op* {vCCCC .. vNNNN}, meth@BBBB, proto@HHHH | invoke-polymorphic/range |
| AA\|*op* BBBBlo BBBB BBBB BBBBhi | 51l | *op* vAA, #+BBBBBBBBBBBBBBBB | const-wide |
