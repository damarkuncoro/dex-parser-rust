use serde::Serialize;
use std::collections::HashMap;
use crate::analysis::core::utils::merge_hashmaps_with_vecs;

#[derive(Serialize, Debug, Default, Clone)]
pub struct XrefMap {
    pub method_to_methods: HashMap<String, Vec<String>>,
    pub method_to_fields: HashMap<String, Vec<String>>,
    pub method_to_strings: HashMap<String, Vec<String>>,
    pub method_to_types: HashMap<String, Vec<String>>,
}

impl XrefMap {
    pub fn merge(&mut self, other: XrefMap) {
        merge_hashmaps_with_vecs(&mut self.method_to_methods, other.method_to_methods);
        merge_hashmaps_with_vecs(&mut self.method_to_fields, other.method_to_fields);
        merge_hashmaps_with_vecs(&mut self.method_to_strings, other.method_to_strings);
        merge_hashmaps_with_vecs(&mut self.method_to_types, other.method_to_types);
    }

    pub fn deduplicate(&mut self) {
        for list in self.method_to_methods.values_mut() { list.sort(); list.dedup(); }
        for list in self.method_to_fields.values_mut() { list.sort(); list.dedup(); }
        for list in self.method_to_strings.values_mut() { list.sort(); list.dedup(); }
        for list in self.method_to_types.values_mut() { list.sort(); list.dedup(); }
    }
}
