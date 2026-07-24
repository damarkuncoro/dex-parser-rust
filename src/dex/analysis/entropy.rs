pub struct EntropyAnalyzer;

impl EntropyAnalyzer {
    /// Calculates Shannon Entropy for a byte slice.
    /// Result is between 0.0 (totally predictable) and 8.0 (completely random/encrypted).
    pub fn calculate(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

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

    /// Analyzes gaps in a DEX file and identifies suspicious high-entropy regions.
    pub fn analyze_gaps(buffer: &[u8], gaps: &[(usize, usize)]) -> Vec<GapAnalysis> {
        gaps.iter()
            .map(|&(offset, length)| {
                let end = (offset + length).min(buffer.len());
                let data = &buffer[offset..end];
                let entropy = Self::calculate(data);

                GapAnalysis {
                    offset,
                    length,
                    entropy,
                    is_suspicious: entropy > 6.5 && length > 100, // Thresholds for suspicious data
                }
            })
            .collect()
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct GapAnalysis {
    pub offset: usize,
    pub length: usize,
    pub entropy: f64,
    pub is_suspicious: bool,
}
