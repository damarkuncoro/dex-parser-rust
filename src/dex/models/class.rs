use serde::Serialize;
use super::method::EncodedMethod;
use super::annotation::AnnotationsDirectory;
use super::encoded_value::EncodedValue;

#[derive(Serialize)]
pub struct Class<'a> {
    pub class_idx: u32,
    pub name: &'a str,
    pub access_flags: u32,
    pub access_flags_text: String,
    pub superclass: &'a str,
    pub interfaces: Vec<&'a str>,
    pub source_file_idx: i32,
    pub source_file: Option<&'a str>,
    pub static_fields: Vec<EncodedField<'a>>,
    pub instance_fields: Vec<EncodedField<'a>>,
    pub direct_methods: Vec<EncodedMethod<'a>>,
    pub virtual_methods: Vec<EncodedMethod<'a>>,
    pub annotations: Option<AnnotationsDirectory<'a>>,
    pub static_values: Vec<EncodedValue<'a>>,
}

#[derive(Serialize)]
pub struct EncodedField<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
    pub access_flags: u32,
    pub access_flags_text: String,
}

#[derive(Serialize)]
pub struct Code<'a> {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub insns_size: u32,
    pub instructions: Vec<Instruction>,
    pub catches: Vec<CatchHandler<'a>>,
    pub debug_info: Option<DebugInfo<'a>>,
}

#[derive(Serialize)]
pub struct CatchHandler<'a> {
    pub start_addr: u32,
    pub end_addr: u32,
    pub handlers: Vec<TryHandler<'a>>,
}

#[derive(Serialize)]
pub struct TryHandler<'a> {
    pub type_name: &'a str,
    pub addr: u32,
}

#[derive(Serialize)]
pub struct Instruction {
    pub offset: usize,
    pub opcode: u8,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct DebugInfo<'a> {
    pub line_start: u32,
    pub parameters: Vec<Option<&'a str>>,
    pub entries: Vec<DebugEntry<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub enum DebugEntry<'a> {
    LineNumber { address_diff: u32, line_diff: i32 },
    StartLocal { address_diff: u32, name: &'a str, type_name: &'a str },
    EndLocal { address_diff: u32 },
    RestartLocal { address_diff: u32 },
}
