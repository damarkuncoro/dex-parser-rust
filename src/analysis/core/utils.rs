use crate::dex::core::models::Instruction;
use crate::dex::core::instructions::opcodes::{OpcodeTable, IndexType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Reference<'a> {
    Method(&'a str),
    Field(&'a str),
    String(&'a str),
    Type(&'a str),
    CallSite(&'a str),
    MethodHandle(&'a str),
    Proto(&'a str),
}

pub struct ReferenceExtractor;

impl ReferenceExtractor {
    /// Extracts a reference from an instruction using its opcode metadata.
    pub fn extract(ins: &Instruction) -> Option<Reference<'_>> {
        let resolved = ins.resolved_value.as_deref()?;
        let info = OpcodeTable::get(ins.opcode);

        match info.index_type {
            IndexType::Method => Some(Reference::Method(resolved)),
            IndexType::Field => Some(Reference::Field(resolved)),
            IndexType::String => Some(Reference::String(resolved)),
            IndexType::Type => Some(Reference::Type(resolved)),
            IndexType::CallSite => Some(Reference::CallSite(resolved)),
            IndexType::MethodHandle => Some(Reference::MethodHandle(resolved)),
            IndexType::Proto => Some(Reference::Proto(resolved)),
            IndexType::None => None,
        }
    }
}

/// Helper for merging two HashMaps that contain Vecs.
pub fn merge_hashmaps_with_vecs<K, V>(
    target: &mut HashMap<K, Vec<V>>,
    source: HashMap<K, Vec<V>>,
) where
    K: std::hash::Hash + Eq,
{
    for (k, mut v) in source {
        target.entry(k).or_default().append(&mut v);
    }
}
