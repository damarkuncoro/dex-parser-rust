# Structural Bytecode Constraints

Constraints on relationships between several elements of the bytecode, usually requiring control or data-flow analysis.

| ID | Description |
| :--- | :--- |
| **B1** | Number and types of arguments must match the instruction. |
| **B2** | Register pairs must never be broken up. |
| **B3** | A register must be assigned before it is read. |
| **B4** | `invoke-direct` must invoke an initializer or a method in current class or superclass. |
| **B5** | Instance initializer must be called only on an uninitialized instance. |
| **B6** | Instance methods/fields can only be accessed on initialized instances. |
| **B7** | `new-instance` result register must not be used if the instruction is re-executed before initialization. |
| **B8** | Initializers must call another initializer (same/superclass) before member access. |
| **B9** | Actual arguments must be assignment-compatible with formal arguments. |
| **B10** | For instance methods, the instance must be compatible with the specified class/interface. |
| **B11** | `return<kind>` must match the method's return type. |
| **B12** | Accessing protected superclass members requires the instance type to be current class or subclass. |
| **B13** | Static field store value must be assignment-compatible with field type. |
| **B14** | Field store value must be assignment-compatible with field type. |
| **B15** | Array store value must be assignment-compatible with array component type. |
| **B16** | `throw` operand must be a `java.lang.Throwable`. |
| **B17** | Last reachable instruction must be a branch, return, or throw. Control flow cannot fall off the end. |
| **B18** | Unassigned half of a former register pair is invalid. |
| **B19** | `move-result<kind>` must immediately follow an `invoke-<kind>` (or `filled-new-array` for object). |
| **B20** | `move-result<kind>` must be preceded in actual flow by a matching `return` or result-producing op. |
| **B21** | `move-exception` must be the first instruction in an exception handler. |
| **B22** | Switch and array data payloads must not be reachable by normal control flow. |
