# Method Invocation Opcodes

Calling methods (virtual, static, direct, etc.).

| Opcode | Name | Description |
|--------|------|-------------|
| 0x6e | OP_INVOKE_VIRTUAL | Invoke a normal virtual method. |
| 0x6f | OP_INVOKE_SUPER | Invoke a superclass virtual method. |
| 0x70 | OP_INVOKE_DIRECT | Invoke a non-static direct method. |
| 0x71 | OP_INVOKE_STATIC | Invoke a static method. |
| 0x72 | OP_INVOKE_INTERFACE | Invoke an interface method. |
| 0x74 | OP_INVOKE_VIRTUAL_RANGE | Invoke a virtual method (range). |
| 0x75 | OP_INVOKE_SUPER_RANGE | Invoke a superclass virtual method (range). |
| 0x76 | OP_INVOKE_DIRECT_RANGE | Invoke a direct method (range). |
| 0x77 | OP_INVOKE_STATIC_RANGE | Invoke a static method (range). |
| 0x78 | OP_INVOKE_INTERFACE_RANGE | Invoke an interface method (range). |
| **Advanced (API 26+)** | | |
| 0xfa | OP_INVOKE_POLYMORPHIC | Invoke a signature polymorphic method. |
| 0xfb | OP_INVOKE_POLYMORPHIC_RANGE | Invoke a signature polymorphic method (range). |
| 0xfc | OP_INVOKE_CUSTOM | Resolve and invoke a call site. |
| 0xfd | OP_INVOKE_CUSTOM_RANGE | Resolve and invoke a call site (range). |

## Details

### invoke-kind
- `virtual`: For normal methods (not static, private, or constructor).
- `super`: For calling parent implementation.
- `direct`: For private methods or constructors.
- `static`: For static methods.
- `interface`: For interface methods where concrete class is unknown.

### invoke-polymorphic
Introduced for `MethodHandle.invoke` and `MethodHandle.invokeExact`.

### Quickened Invokes
| Opcode | Name | Description |
|--------|------|-------------|
| 0xf8 | OP_INVOKE_VIRTUAL_QUICK | Optimized virtual method call. |
| 0xf9 | OP_INVOKE_VIRTUAL_QUICK_RANGE | Optimized virtual method call (range). |
