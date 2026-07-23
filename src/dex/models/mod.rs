pub mod header;
pub mod class;
pub mod method;
pub mod field;
pub mod type_id;
pub mod proto;
pub mod raw;

pub use header::RawHeader;
pub use class::{Class, Code, Instruction, EncodedField, CatchHandler, TryHandler};
pub use method::EncodedMethod;
pub use field::Field;
pub use proto::Proto;

use serde::Serialize;

#[derive(Serialize)]
pub struct Dex {
    pub header: RawHeader,
    pub strings: Vec<String>,
    pub types: Vec<String>,
    pub protos: Vec<Proto>,
    pub fields: Vec<Field>,
    pub methods: Vec<String>,
    pub classes: Vec<Class>,
}

impl Dex {
    pub fn get_string(&self, idx: u32) -> Option<&String> {
        self.strings.get(idx as usize)
    }

    pub fn get_type(&self, idx: u32) -> Option<&String> {
        self.types.get(idx as usize)
    }

    pub fn get_method(&self, idx: u32) -> Option<&String> {
        self.methods.get(idx as usize)
    }

    pub fn get_field(&self, idx: u32) -> Option<&Field> {
        self.fields.get(idx as usize)
    }
}
