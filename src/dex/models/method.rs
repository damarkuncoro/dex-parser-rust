use scroll::Pread;
use serde::Serialize;
use super::class::Code;

#[derive(Debug, Pread, Serialize)]
pub struct RawMethodId {
    pub class_idx: u16,
    pub proto_idx: u16,
    pub name_idx: u32,
}

#[derive(Serialize)]
pub struct EncodedMethod {
    pub name: String,
    pub signature: String, // Contoh: ([B[B)[B
    pub access_flags: u32,
    pub access_flags_text: String, // Contoh: PUBLIC STATIC
    pub code_off: u64,
    pub code: Option<Code>,
}
