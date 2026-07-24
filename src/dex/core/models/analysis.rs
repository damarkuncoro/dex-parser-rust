use serde::Serialize;
use crate::dex::analysis::entropy::GapAnalysis;
use crate::dex::analysis::scanner::ScanResult;

#[derive(Serialize, Default, Clone)]
pub struct AnalysisReport {
    /// Temuan berdasarkan analisis entropi pada celah data.
    pub forensic_gaps: Vec<GapAnalysis>,
    /// Indikator data sensitif yang ditemukan oleh scanner.
    pub sensitive_indicators: Vec<ScanResult>,
    /// Statistik keamanan umum.
    pub stats: AnalysisStats,
}

#[derive(Serialize, Default, Clone)]
pub struct AnalysisStats {
    pub total_gap_size: usize,
    pub suspicious_gap_count: usize,
    pub max_entropy: f64,
    pub sensitive_count: usize,
}

impl AnalysisReport {
    pub fn new(gaps: Vec<GapAnalysis>, indicators: Vec<ScanResult>) -> Self {
        let total_gap_size = gaps.iter().map(|g| g.length).sum();
        let suspicious_gap_count = gaps.iter().filter(|g| g.is_suspicious).count();
        let max_entropy = gaps.iter().map(|g| g.entropy).fold(0.0, f64::max);
        let sensitive_count = indicators.len();

        Self {
            forensic_gaps: gaps,
            sensitive_indicators: indicators,
            stats: AnalysisStats {
                total_gap_size,
                suspicious_gap_count,
                max_entropy,
                sensitive_count,
            },
        }
    }
}
