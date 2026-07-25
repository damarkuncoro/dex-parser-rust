use serde::Serialize;
use crate::dex::analysis::scanner::ScanResult;

#[derive(Serialize)]
pub struct DexSummary {
    pub name: String,
    pub magic: String,
    pub class_count: usize,
    pub gap_count: usize,
    pub total_gap_size: usize,
    pub suspicious_gap_count: usize,
    pub max_entropy: f64,
    pub sensitive_string_count: usize,
}

#[derive(Serialize)]
pub struct WasmLoadResult {
    pub summaries: Vec<DexSummary>,
    pub class_names: Vec<Vec<String>>,
    pub scan_results: Vec<Vec<ScanResult>>,
    pub global_intelligence: crate::dex::analysis::GlobalIntelligence,
}
