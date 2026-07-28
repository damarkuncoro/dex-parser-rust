use serde::Serialize;
use std::collections::HashMap;
use crate::analysis::core::utils::merge_hashmaps_with_vecs;

#[derive(Serialize, Default, Debug, Clone)]
pub struct GlobalIntelligence {
    pub cross_dex_calls: HashMap<String, Vec<CallSite>>,
    pub cross_dex_fields: HashMap<String, Vec<CallSite>>,
    pub cross_dex_strings: HashMap<String, Vec<CallSite>>,
    pub cross_dex_types: HashMap<String, Vec<CallSite>>,
    pub global_security_summary: GlobalSecuritySummary,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallSite {
    pub dex_name: String,
    pub class_name: String,
    pub method_name: String,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct GlobalSecuritySummary {
    pub total_suspicious_gaps: usize,
    pub total_sensitive_indicators: usize,
    pub potentially_packed: bool,
}

impl GlobalIntelligence {
    pub fn merge(&mut self, other: GlobalIntelligence) {
        merge_hashmaps_with_vecs(&mut self.cross_dex_calls, other.cross_dex_calls);
        merge_hashmaps_with_vecs(&mut self.cross_dex_fields, other.cross_dex_fields);
        merge_hashmaps_with_vecs(&mut self.cross_dex_strings, other.cross_dex_strings);
        merge_hashmaps_with_vecs(&mut self.cross_dex_types, other.cross_dex_types);

        self.global_security_summary.total_suspicious_gaps += other.global_security_summary.total_suspicious_gaps;
        self.global_security_summary.total_sensitive_indicators += other.global_security_summary.total_sensitive_indicators;
        self.global_security_summary.potentially_packed |= other.global_security_summary.potentially_packed;
    }

    pub fn deduplicate(&mut self) {
        for list in self.cross_dex_calls.values_mut() { list.sort(); list.dedup(); }
        for list in self.cross_dex_fields.values_mut() { list.sort(); list.dedup(); }
        for list in self.cross_dex_strings.values_mut() { list.sort(); list.dedup(); }
        for list in self.cross_dex_types.values_mut() { list.sort(); list.dedup(); }
    }
}
