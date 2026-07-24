use crate::dex::core::constants::access_flags as masks;

pub fn translate_access_flags(flags: u32, context_is_method: bool) -> String {
    let mut result = Vec::new();

    // 1. Visibility
    if flags & masks::PUBLIC != 0 { result.push("PUBLIC"); }
    if flags & masks::PRIVATE != 0 { result.push("PRIVATE"); }
    if flags & masks::PROTECTED != 0 { result.push("PROTECTED"); }

    // 2. Modifiers
    if flags & masks::STATIC != 0 { result.push("STATIC"); }
    if flags & masks::FINAL != 0 { result.push("FINAL"); }
    if flags & masks::ABSTRACT != 0 { result.push("ABSTRACT"); }

    // 3. Method/Field Specific
    if flags & masks::NATIVE != 0 { result.push("NATIVE"); }

    if flags & masks::SYNCHRONIZED != 0 {
        if context_is_method {
            result.push("SYNCHRONIZED");
        }
    }

    if flags & masks::VOLATILE != 0 { // Also masks::BRIDGE
        if context_is_method { result.push("BRIDGE"); }
        else { result.push("VOLATILE"); }
    }

    if flags & masks::TRANSIENT != 0 { // Also masks::VARARGS
        if context_is_method { result.push("VARARGS"); }
        else { result.push("TRANSIENT"); }
    }

    // 4. Class Specific
    if flags & masks::INTERFACE != 0 { result.push("INTERFACE"); }
    if flags & masks::ANNOTATION != 0 { result.push("ANNOTATION"); }
    if flags & masks::ENUM != 0 { result.push("ENUM"); }

    // 5. Advanced / Internal
    if flags & masks::STRICTFP != 0 { result.push("STRICTFP"); }
    if flags & masks::SYNTHETIC != 0 { result.push("SYNTHETIC"); }
    if flags & masks::CONSTRUCTOR != 0 { result.push("CONSTRUCTOR"); }
    if flags & masks::DECLARED_SYNCHRONIZED != 0 { result.push("DECLARED_SYNCHRONIZED"); }

    if result.is_empty() {
        "".to_string()
    } else {
        result.join(" | ")
    }
}
