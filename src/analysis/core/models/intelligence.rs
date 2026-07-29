use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::forensic::ScanResult;
use super::xref::{XrefMap, CallSite};
use super::report::RiskLevel;
use crate::analysis::core::utils::merge_hashmaps_with_vecs;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GlobalIntelligence {
    pub cross_dex_calls: HashMap<String, Vec<CallSite>>,
    pub cross_dex_fields: HashMap<String, Vec<CallSite>>,
    pub cross_dex_strings: HashMap<String, Vec<CallSite>>,
    pub cross_dex_types: HashMap<String, Vec<CallSite>>,
    pub behavioral_indicators: Vec<ScanResult>,
    pub global_security_summary: GlobalSecuritySummary,
    /// High-level intelligence tags (e.g., "Potential Spyware", "Adware")
    pub analysis_tags: Vec<AnalysisTag>,
    /// Maps Resource IDs found in code to their names from resources.arsc
    pub resolved_resources: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnalysisTag {
    pub name: String,
    pub description: String,
    pub severity: RiskLevel,
    pub mitre_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GlobalSecuritySummary {
    pub total_suspicious_gaps: usize,
    pub total_sensitive_indicators: usize,
    pub total_spec_violations: usize,
    pub total_dead_code: usize,
    pub potentially_packed: bool,
}

impl GlobalIntelligence {
    /// Helper to merge all XREFs from all DEX files into a single map.
    pub fn merge_all_xrefs(&self, apk: &crate::dex::core::models::Apk) -> XrefMap {
        let mut total_xref = XrefMap::default();
        for dex in &apk.dex_files {
            total_xref.merge(dex.analysis.xrefs.clone());
        }
        total_xref
    }

    pub fn merge(&mut self, other: GlobalIntelligence) {
        merge_hashmaps_with_vecs(&mut self.cross_dex_calls, other.cross_dex_calls);
        merge_hashmaps_with_vecs(&mut self.cross_dex_fields, other.cross_dex_fields);
        merge_hashmaps_with_vecs(&mut self.cross_dex_strings, other.cross_dex_strings);
        merge_hashmaps_with_vecs(&mut self.cross_dex_types, other.cross_dex_types);

        self.behavioral_indicators.extend(other.behavioral_indicators);

        self.global_security_summary.total_suspicious_gaps += other.global_security_summary.total_suspicious_gaps;
        self.global_security_summary.total_sensitive_indicators += other.global_security_summary.total_sensitive_indicators;
        self.global_security_summary.total_spec_violations += other.global_security_summary.total_spec_violations;
        self.global_security_summary.total_dead_code += other.global_security_summary.total_dead_code;
        self.global_security_summary.potentially_packed |= other.global_security_summary.potentially_packed;

        self.analysis_tags.extend(other.analysis_tags);
        self.resolved_resources.extend(other.resolved_resources);
    }

    pub fn deduplicate(&mut self) {
        for list in self.cross_dex_calls.values_mut() { deduplicate_sites(list); }
        for list in self.cross_dex_fields.values_mut() { deduplicate_sites(list); }
        for list in self.cross_dex_strings.values_mut() { deduplicate_sites(list); }
        for list in self.cross_dex_types.values_mut() { list.sort(); list.dedup(); }

        self.behavioral_indicators.sort_by(|a, b| a.content.cmp(&b.content));
        self.behavioral_indicators.dedup_by(|a, b| a.content == b.content);

        self.analysis_tags.sort_by(|a, b| a.name.cmp(&b.name));
        self.analysis_tags.dedup_by(|a, b| a.name == b.name);
    }
}

fn deduplicate_sites(sites: &mut Vec<CallSite>) {
    sites.sort_by(|a, b| {
        a.dex_name.cmp(&b.dex_name)
            .then(a.class_name.cmp(&b.class_name))
            .then(a.method_name.cmp(&b.method_name))
            .then(a.method_signature.cmp(&b.method_signature))
    });
    sites.dedup();
}
