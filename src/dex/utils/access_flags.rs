pub fn translate_access_flags(flags: u32, context_is_method: bool) -> String {
    let mut result = Vec::new();

    // Common flags
    if flags & 0x0001 != 0 { result.push("PUBLIC"); }
    if flags & 0x0002 != 0 { result.push("PRIVATE"); }
    if flags & 0x0004 != 0 { result.push("PROTECTED"); }
    if flags & 0x0008 != 0 { result.push("STATIC"); }
    if flags & 0x0010 != 0 { result.push("FINAL"); }

    // Class/Field/Method specific
    if flags & 0x0020 != 0 {
        if context_is_method {
            result.push("SYNCHRONIZED");
        } else {
            // For classes, 0x20 is usually SUPER (acc_super)
            // But dexdump often doesn't show it as text unless it's a specific flag
        }
    }

    if flags & 0x0040 != 0 {
        if context_is_method {
            result.push("BRIDGE");
        } else {
            result.push("VOLATILE");
        }
    }

    if flags & 0x0080 != 0 {
        if context_is_method {
            result.push("VARARGS");
        } else {
            result.push("TRANSIENT");
        }
    }

    if flags & 0x0100 != 0 { result.push("NATIVE"); }
    if flags & 0x0200 != 0 { result.push("INTERFACE"); }
    if flags & 0x0400 != 0 { result.push("ABSTRACT"); }
    if flags & 0x0800 != 0 { result.push("STRICTFP"); }
    if flags & 0x1000 != 0 { result.push("SYNTHETIC"); }
    if flags & 0x2000 != 0 { result.push("ANNOTATION"); }
    if flags & 0x4000 != 0 { result.push("ENUM"); }
    if flags & 0x10000 != 0 { result.push("CONSTRUCTOR"); }
    if flags & 0x20000 != 0 { result.push("DECLARED_SYNCHRONIZED"); }

    if result.is_empty() {
        "".to_string()
    } else {
        result.join(" ")
    }
}
