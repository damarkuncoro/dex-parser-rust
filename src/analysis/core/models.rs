use serde::Serialize;
use std::collections::HashMap;
use crate::analysis::core::utils::merge_hashmaps_with_vecs;

// --- Forensic Models ---

#[derive(Debug, Serialize, Clone)]
pub struct GapAnalysis {
    pub offset: usize,
    pub length: usize,
    pub entropy: f64,
    pub is_suspicious: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ScanResult {
    pub category: String,
    pub content: String,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct AnalysisReport {
    pub forensic_gaps: Vec<GapAnalysis>,
    pub sensitive_indicators: Vec<ScanResult>,
    pub xrefs: XrefMap,
    pub method_tokens: HashMap<String, Vec<AnalysisToken>>,
    pub stats: AnalysisStats,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct AnalysisStats {
    pub total_gap_size: usize,
    pub suspicious_gap_count: usize,
    pub max_entropy: f64,
    pub sensitive_count: usize,
    pub total_methods_analyzed: usize,
    pub total_instructions_scanned: usize,
    pub call_count: usize,
    pub jump_count: usize,
    pub string_count: usize,
}

impl AnalysisReport {
    pub fn new(
        gaps: Vec<GapAnalysis>,
        indicators: Vec<ScanResult>,
        xrefs: XrefMap,
        method_tokens: HashMap<String, Vec<AnalysisToken>>,
        total_instructions: usize
    ) -> Self {
        let total_gap_size = gaps.iter().map(|g| g.length).sum();
        let suspicious_gap_count = gaps.iter().filter(|g| g.is_suspicious).count();
        let max_entropy = gaps.iter().map(|g| g.entropy).fold(0.0, f64::max);
        let sensitive_count = indicators.len();
        let total_methods_analyzed = xrefs.method_to_methods.len();

        Self {
            forensic_gaps: gaps,
            sensitive_indicators: indicators,
            xrefs,
            method_tokens,
            stats: AnalysisStats {
                total_gap_size,
                suspicious_gap_count,
                max_entropy,
                sensitive_count,
                total_methods_analyzed,
                total_instructions_scanned: total_instructions,
                call_count: 0,
                jump_count: 0,
                string_count: 0,
            },
        }
    }
}

// --- Tokenization Models ---

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AnalysisToken {
    ExternalCall(String),
    InternalCall(String),
    StringUsage(String),
    CryptoOp(String),
    NativeLoad(String),
    DynamicLoad,
    Reflection,
    SystemCommand(String),
}

// --- Control Flow Models ---

#[derive(Serialize, Debug, Clone)]
pub struct BasicBlock {
    pub start_offset: usize,
    pub end_offset: usize,
    pub instructions: Vec<usize>,
    pub successors: Vec<usize>,
}

// --- Cross-Reference Models ---

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

// --- Global Intelligence Models (APK Level) ---

#[derive(Serialize, Default, Debug, Clone)]
pub struct GlobalIntelligence {
    pub cross_dex_calls: HashMap<String, Vec<CallSite>>,
    pub cross_dex_fields: HashMap<String, Vec<CallSite>>,
    pub cross_dex_strings: HashMap<String, Vec<CallSite>>,
    pub cross_dex_types: HashMap<String, Vec<CallSite>>,
    pub behavioral_indicators: Vec<ScanResult>,
    pub global_security_summary: GlobalSecuritySummary,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallSite {
    pub dex_name: String,
    pub class_name: String,
    pub method_name: String,
    pub method_signature: String,
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

        self.behavioral_indicators.extend(other.behavioral_indicators);

        self.global_security_summary.total_suspicious_gaps += other.global_security_summary.total_suspicious_gaps;
        self.global_security_summary.total_sensitive_indicators += other.global_security_summary.total_sensitive_indicators;
        self.global_security_summary.potentially_packed |= other.global_security_summary.potentially_packed;
    }

    pub fn deduplicate(&mut self) {
        for list in self.cross_dex_calls.values_mut() { deduplicate_sites(list); }
        for list in self.cross_dex_fields.values_mut() { deduplicate_sites(list); }
        for list in self.cross_dex_strings.values_mut() { deduplicate_sites(list); }
        for list in self.cross_dex_types.values_mut() { list.sort(); list.dedup(); }

        self.behavioral_indicators.sort_by(|a, b| a.content.cmp(&b.content));
        self.behavioral_indicators.dedup_by(|a, b| a.content == b.content);
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
