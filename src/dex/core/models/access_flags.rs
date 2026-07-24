pub struct AccessFlags(pub u32);

impl AccessFlags {
    pub fn is_public(&self) -> bool { self.0 & 0x0001 != 0 }
    pub fn is_static(&self) -> bool { self.0 & 0x0008 != 0 }
    pub fn is_final(&self) -> bool { self.0 & 0x0010 != 0 }

    pub fn to_string(&self) -> String {
        let mut result = Vec::new();
        if self.is_public() { result.push("PUBLIC"); }
        if self.is_static() { result.push("STATIC"); }
        if self.is_final() { result.push("FINAL"); }

        if result.is_empty() { "NONE".to_string() } else { result.join(" ") }
    }
}
