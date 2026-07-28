# Static Bytecode Constraints

Constraints on individual elements of the bytecode that can be checked without control or data-flow analysis.

| ID | Description |
| :--- | :--- |
| **A1** | The `insns` array must not be empty. |
| **A2** | The first opcode in `insns` must have index zero. |
| **A3** | `insns` must contain only valid Dalvik opcodes. |
| **A4** | Instruction indices must be consistent with instruction lengths. |
| **A5** | The last instruction must end at `insns_size - 1`. |
| **A6** | `goto` and `if-<kind>` targets must be within the same method. |
| **A7** | `packed-switch` targets must be within the same method. |
| **A8** | `sparse-switch` targets must be within the same method. Table must be sorted low-to-high. |
| **A9** | `const-string` operand must be a valid index into string pool. |
| **A10** | `iget/iput` field index must be valid and represent an instance field. |
| **A11** | `sget/sput` field index must be valid and represent a static field. |
| **A12** | `invoke-virtual`, `invoke-super`, `invoke-direct`, `invoke-static` method index must be valid. |
| **A13** | `invoke-*/range` method index must be valid. |
| **A14** | Methods starting with `<` (except `<init>`) must only be invoked implicitly by the VM. |
| **A15** | `invoke-interface` method must belong to an interface. |
| **A16** | `invoke-interface/range` method must belong to an interface. |
| **A17** | `const-class`, `check-cast`, `new-instance`, `filled-new-array/range` type index must be valid. |
| **A18** | `instance-of`, `new-array`, `filled-new-array` type index must be valid. |
| **A19** | `new-array` dimensions must be less than 256. |
| **A20** | `new` instruction must not refer to array classes, interfaces, or abstract classes. |
| **A21** | `new-array` type must be a valid, non-reference type. |
| **A22** | Single-width register indices must be less than `registers_size`. |
| **A23** | Double-width register indices must be less than `registers_size - 1`. |
| **A24** | `invoke-virtual/direct` method must belong to a class (not interface). |
| **A25** | `invoke-virtual/direct/range` method must belong to a class. |
