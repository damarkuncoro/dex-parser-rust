use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GapAnalysis {
    pub offset: usize,
    pub length: usize,
    pub entropy: f64,
    pub is_suspicious: bool,
    pub is_null_padded: bool,
    pub data_preview: String,
}

use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GlobalTaintSummary {
    /// Methods that return sensitive data (Sources or wrappers of Sources)
    pub source_returners: HashSet<String>,
    /// Methods that eventually lead to a data leak
    pub leaking_methods: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ScanResult {
    pub category: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<CryptoDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoDetails {
    pub algorithm: String,
    pub mode: String,
    pub padding: String,
    pub risk: String,
    pub reason: String,
}
