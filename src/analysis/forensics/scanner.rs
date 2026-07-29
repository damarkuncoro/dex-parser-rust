use crate::analysis::core::models::ScanResult;
use crate::analysis::core::config::CompiledConfig;
use rayon::prelude::*;

pub struct StringScanner;

impl StringScanner {
    pub fn scan(strings: &[&[u8]], compiled: &CompiledConfig) -> Vec<ScanResult> {
        let mut results: Vec<ScanResult> = strings.par_iter()
            .filter(|bytes| bytes.len() >= compiled.config.forensics.min_string_length)
            .flat_map(|&bytes| {
                let mut local = Vec::new();

                // Regex-based rules
                for (category, regex) in compiled.scanner_regex.iter() {
                    for mat in regex.find_iter(bytes) {
                        local.push(ScanResult {
                            category: category.to_string(),
                            content: String::from_utf8_lossy(mat.as_bytes()).to_string(),
                            details: None,
                        });
                    }
                }

                // Heuristic-based rules
                for rule in &compiled.config.sensitive_heuristics {
                    if bytes.windows(rule.pattern.len()).any(|w| w == rule.pattern.as_bytes()) && bytes.len() > rule.min_len {
                         local.push(ScanResult {
                            category: rule.category.clone(),
                            content: String::from_utf8_lossy(bytes).to_string(),
                            details: None,
                        });
                    }
                }

                // Long string detection
                if bytes.len() > compiled.config.forensics.suspicious_string_length {
                    local.push(ScanResult {
                        category: "Suspiciously Long String".to_string(),
                        content: format!("Len: {} | Start: {}", bytes.len(), String::from_utf8_lossy(&bytes[..128.min(bytes.len())])),
                        details: None,
                    });
                }

                local
            })
            .collect();

        results.sort_by(|a, b| a.content.cmp(&b.content));
        results.dedup_by(|a, b| a.content == b.content);

        results
    }
}
