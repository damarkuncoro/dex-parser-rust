use scroll::Pread;
use serde::Serialize;

#[derive(Debug, Pread, Clone, Serialize)]
pub struct RawFieldId {
    pub class_idx: u16,
    pub type_idx: u16,
    pub name_idx: u32,
}

#[derive(Clone, Serialize)]
pub struct Field<'a> {
    pub class: &'a str,
    pub type_name: &'a str,
    pub name: &'a str,
}
