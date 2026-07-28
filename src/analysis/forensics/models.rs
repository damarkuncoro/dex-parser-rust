use serde::Serialize;

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
