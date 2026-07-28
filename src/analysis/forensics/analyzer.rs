use crate::analysis::core::models::{GapAnalysis, ScanResult};
use crate::analysis::forensics::entropy::EntropyAnalyzer;
use crate::analysis::forensics::scanner::StringScanner;
use crate::analysis::core::config::CompiledConfig;

pub struct ForensicAnalyzer;

impl ForensicAnalyzer {
    pub fn run(buffer: &[u8], strings: &[&[u8]], gaps: &[(usize, usize)], compiled: &CompiledConfig) -> (Vec<GapAnalysis>, Vec<ScanResult>) {
        rayon::join(
            || EntropyAnalyzer::analyze_gaps(buffer, gaps, &compiled.config.forensics),
            || StringScanner::scan(strings, compiled)
        )
    }
}
