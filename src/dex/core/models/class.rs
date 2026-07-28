use serde::{Serialize};
use super::method::EncodedMethod;
use super::annotation::AnnotationsDirectory;
use super::encoded_value::EncodedValue;

#[derive(Serialize, Default)]
pub struct Class<'a> {
    pub class_idx: u32,
    pub name: String,
    pub access_flags: u32,
    pub access_flags_text: String,
    pub superclass: String,
    pub interfaces: Vec<String>,
    pub source_file_idx: i32,
    pub source_file: Option<String>,
    pub static_fields: Vec<EncodedField<'a>>,
    pub instance_fields: Vec<EncodedField<'a>>,
    pub direct_methods: Vec<EncodedMethod<'a>>,
    pub virtual_methods: Vec<EncodedMethod<'a>>,
    pub annotations: Option<AnnotationsDirectory<'a>>,
    pub static_values: Vec<EncodedValue<'a>>,
}

#[derive(Serialize, Default)]
pub struct EncodedField<'a> {
    pub name: String,
    pub type_name: String,
    pub access_flags: u32,
    pub access_flags_text: String,
    #[serde(skip)] pub _marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Serialize, Default)]
pub struct Code<'a> {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub insns_size: u32,
    pub instructions: Vec<Instruction>,
    pub catches: Vec<CatchHandler<'a>>,
    pub debug_info: Option<DebugInfo<'a>>,
}

#[derive(Serialize, Default)]
pub struct CatchHandler<'a> {
    pub start_addr: u32,
    pub end_addr: u32,
    pub handlers: Vec<TryHandler<'a>>,
}

#[derive(Serialize, Default)]
pub struct TryHandler<'a> {
    pub type_name: String,
    pub addr: u32,
    #[serde(skip)] pub _marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Serialize, Default)]
pub struct Instruction {
    pub offset: usize,
    pub opcode: u8,
    pub name: String,
    pub description: String,
    pub index: Option<u32>,
    pub resolved_value: Option<String>,
    pub registers: Vec<u16>,
    pub target_offset: Option<u32>,
    pub immediates: Vec<u64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DebugInfo<'a> {
    pub line_start: u32,
    pub parameters: Vec<Option<String>>,
    pub entries: Vec<DebugEntry<'a>>,
    #[serde(skip)] pub _marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Serialize, Clone, Debug)]
pub enum DebugEntry<'a> {
    AdvancePc { addr_diff: u32 },
    AdvanceLine { line_diff: i32 },
    StartLocal { register_num: u32, name: String, type_name: String },
    StartLocalExtended { register_num: u32, name: String, type_name: String, signature: String },
    EndLocal { register_num: u32 },
    RestartLocal { register_num: u32 },
    SetPrologueEnd,
    SetEpilogueBegin,
    SetFile { name: String },
    SpecialOpcode { opcode: u8, line_diff: i32, addr_diff: u32 },
    #[serde(skip)] _Marker(std::marker::PhantomData<&'a ()>),
}
