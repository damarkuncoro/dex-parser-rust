use scroll::Pread;
use serde::Serialize;
use super::method::EncodedMethod;

#[derive(Debug, Pread, Serialize)]
pub struct RawClassDef {
    pub class_idx: u32,
    pub access_flags: u32,
    pub superclass_idx: u32,
    pub interfaces_off: u32,
    pub source_file_idx: u32,
    pub annotations_off: u32,
    pub class_data_off: u32,
    pub static_values_off: u32,
}

#[derive(Serialize)]
pub struct Class {
    pub name: String,
    pub access_flags: u32,
    pub access_flags_text: String,
    pub superclass: String,
    pub interfaces: Vec<String>,
    pub source_file_idx: i32,
    pub source_file: Option<String>,
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}

#[derive(Serialize)]
pub struct EncodedField {
    pub name: String,
    pub type_name: String,
    pub access_flags: u32,
    pub access_flags_text: String,
}

#[derive(Serialize)]
pub struct Code {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub insns_size: u32,
    pub instructions: Vec<Instruction>,
    pub catches: Vec<CatchHandler>,
}

#[derive(Serialize)]
pub struct CatchHandler {
    pub start_addr: u32,
    pub end_addr: u32,
    pub handlers: Vec<TryHandler>,
}

#[derive(Serialize)]
pub struct TryHandler {
    pub type_name: String,
    pub addr: u32,
}

#[derive(Serialize)]
pub struct Instruction {
    pub offset: usize,
    pub opcode: u8,
    pub name: String,
    pub description: String,
}
