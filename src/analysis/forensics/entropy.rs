use crate::analysis::core::models::GapAnalysis;
use crate::analysis::core::config::ForensicConfig;
use rayon::prelude::*;

pub struct EntropyAnalyzer;

impl EntropyAnalyzer {
    pub fn calculate(data: &[u8]) -> f64 {
        if data.is_empty() { return 0.0; }

        let mut counts = [0usize; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    pub fn analyze_gaps(buffer: &[u8], gaps: &[(usize, usize)], config: &ForensicConfig) -> Vec<GapAnalysis> {
        gaps.par_iter()
            .map(|&(offset, length)| {
                let end = (offset + length).min(buffer.len());
                let data = &buffer[offset..end];
                let entropy = Self::calculate(data);
                let is_null_padded = data.iter().all(|&b| b == 0);

                let data_preview = if data.len() <= 16 {
                    data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
                } else {
                    let hex: String = data[..8].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
                    format!("{}...", hex)
                };

                GapAnalysis {
                    offset,
                    length,
                    entropy,
                    is_suspicious: entropy > config.entropy_threshold
                                && length > config.gap_length_threshold,
                    is_null_padded,
                    data_preview,
                }
            })
            .collect()
    }
}
