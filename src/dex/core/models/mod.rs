pub mod header;
pub mod class;
pub mod method;
pub mod field;
pub mod type_id;
pub mod proto;
pub mod raw;
pub mod encoded_value;
pub mod annotation;
pub mod map_list;
pub mod apk;
pub mod analysis;
pub mod wasm;

pub use header::{RawHeader, ModernHeaderExt};
pub use class::{Class, Code, Instruction, EncodedField, CatchHandler, TryHandler, DebugInfo, DebugEntry};
pub use field::Field;
pub use method::EncodedMethod;
pub use proto::Proto;
pub use encoded_value::{EncodedValue, EncodedAnnotation, AnnotationElement};
pub use annotation::{AnnotationsDirectory, AnnotationItem, FieldAnnotation, MethodAnnotation, ParameterAnnotation};
pub use map_list::{MapList, MapItem};
pub use apk::{Apk, Manifest};
pub use analysis::AnalysisReport;

use crate::dex::parsers::traits::{StringResolver, TypeResolver, MethodResolver, FieldResolver, DexResolver};
use serde::{Serialize};

#[derive(Serialize)]
pub struct Dex<'a> {
    pub header: RawHeader,
    pub metadata: DexMetadata<'a>,
    pub class_defs: Vec<Class<'a>>,
    pub map_list: MapList,
    pub method_handles: Vec<raw::RawMethodHandleItem>,
    pub call_sites: Vec<raw::RawCallSiteIdItem>,
    pub byte_gaps: Vec<(usize, usize)>,
    pub analysis: AnalysisReport,
    #[serde(skip)]
    pub analysis_config: crate::analysis::core::config::AnalysisConfig,
}

#[derive(Serialize, Clone)]
pub struct DexMetadata<'a> {
    pub strings: Vec<&'a [u8]>,
    pub types: Vec<&'a [u8]>,
    pub protos: Vec<Proto<'a>>,
    pub fields: Vec<Field<'a>>,
    pub methods: Vec<String>,
}

impl<'a> StringResolver<'a> for DexMetadata<'a> {
    fn resolve_string(&self, idx: u32) -> Option<&'a [u8]> {
        self.strings.get(idx as usize).copied()
    }
}

impl<'a> TypeResolver<'a> for DexMetadata<'a> {
    fn resolve_type(&self, idx: u32) -> Option<&'a [u8]> {
        self.types.get(idx as usize).copied()
    }
}

impl<'a> MethodResolver for DexMetadata<'a> {
    fn resolve_method(&self, idx: u32) -> Option<String> {
        self.methods.get(idx as usize).cloned()
    }
}

impl<'a> FieldResolver<'a> for DexMetadata<'a> {
    fn resolve_field(&self, idx: u32) -> Option<Field<'a>> {
        self.fields.get(idx as usize).cloned()
    }
}

impl<'a> DexResolver<'a> for DexMetadata<'a> {}
