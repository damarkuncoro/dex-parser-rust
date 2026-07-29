use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::forensic::{GapAnalysis, ScanResult};
use super::xref::XrefMap;
use super::token::AnalysisToken;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct AnalysisReport {
    pub forensic_gaps: Vec<GapAnalysis>,
    pub sensitive_indicators: Vec<ScanResult>,
    pub manifest_indicators: Vec<ScanResult>,
    pub potential_resource_ids: Vec<u32>,
    pub xrefs: XrefMap,
    pub method_tokens: HashMap<String, Vec<AnalysisToken>>,
    pub stats: AnalysisStats,
    pub risk_assessment: RiskAssessment,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct RiskAssessment {
    pub score: f64,
    pub level: RiskLevel,
    pub justifications: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self { RiskLevel::Safe }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct AnalysisStats {
    pub total_gap_size: usize,
    pub suspicious_gap_count: usize,
    pub max_entropy: f64,
    pub sensitive_count: usize,
    pub total_methods_analyzed: usize,
    pub total_instructions_scanned: usize,
    pub unknown_opcodes_count: usize,
    pub spec_violation_count: usize,
    pub unknown_opcodes_distribution: HashMap<u8, usize>,
    pub max_consecutive_nops: usize,
    pub dead_code_count: usize,
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
            manifest_indicators: Vec::new(),
            potential_resource_ids: Vec::new(),
            xrefs,
            method_tokens,
            stats: AnalysisStats {
                total_gap_size,
                suspicious_gap_count,
                max_entropy,
                sensitive_count,
                total_methods_analyzed,
                total_instructions_scanned: total_instructions,
                unknown_opcodes_count: 0,
                spec_violation_count: 0,
                unknown_opcodes_distribution: HashMap::new(),
                max_consecutive_nops: 0,
                dead_code_count: 0,
                call_count: 0,
                jump_count: 0,
                string_count: 0,
            },
            risk_assessment: RiskAssessment::default(),
        }
    }
}
