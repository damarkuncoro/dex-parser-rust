pub mod class;
pub mod field;
pub mod header;
pub mod method;
pub mod proto;
pub mod raw;
pub mod type_id;

pub use class::{CatchHandler, Class, Code, EncodedField, Instruction, TryHandler};
pub use field::Field;
pub use header::RawHeader;
pub use method::EncodedMethod;
pub use proto::Proto;

use serde::Serialize;

/// Represents a fully parsed Android DEX file.
///
/// This structure contains all the metadata, string tables, type definitions,
/// field/method prototypes, and class definitions extracted from the DEX binary.
#[derive(Serialize)]
pub struct Dex {
    /// The original DEX header information.
    pub header: RawHeader,
    /// All strings defined in the DEX file.
    pub strings: Vec<String>,
    /// All type descriptors (e.g., "Ljava/lang/String;").
    pub types: Vec<String>,
    /// Method and field prototypes.
    pub protos: Vec<Proto>,
    /// Field definitions.
    pub fields: Vec<Field>,
    /// Method name/signature descriptors.
    pub methods: Vec<String>,
    /// High-level class definitions, including bytecode.
    pub classes: Vec<Class>,
}

impl Dex {
    /// Resolves a string by its index.
    pub fn get_string(&self, idx: u32) -> Option<&String> {
        self.strings.get(idx as usize)
    }

    /// Resolves a type name by its index.
    pub fn get_type(&self, idx: u32) -> Option<&String> {
        self.types.get(idx as usize)
    }

    /// Resolves a method signature by its index.
    pub fn get_method(&self, idx: u32) -> Option<&String> {
        self.methods.get(idx as usize)
    }

    /// Resolves a field definition by its index.
    pub fn get_field(&self, idx: u32) -> Option<&Field> {
        self.fields.get(idx as usize)
    }
}
