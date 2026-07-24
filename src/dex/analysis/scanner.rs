use regex::bytes::Regex;
use once_cell::sync::Lazy;
use serde::Serialize;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https?://[^\s/$.?#].[^\s]*").unwrap());
static IP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap());
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap());

#[derive(Debug, Serialize, Clone)]
pub struct ScanResult {
    pub category: String,
    pub content: String,
}

pub struct StringScanner;

impl StringScanner {
    pub fn scan(strings: &[&[u8]]) -> Vec<ScanResult> {
        let mut results = Vec::new();

        for &bytes in strings {
            if bytes.len() < 4 { continue; } // Skip very short strings

            // Scan for URLs
            for mat in URL_REGEX.find_iter(bytes) {
                results.push(ScanResult {
                    category: "URL".to_string(),
                    content: String::from_utf8_lossy(mat.as_bytes()).to_string(),
                });
            }

            // Scan for IPs
            for mat in IP_REGEX.find_iter(bytes) {
                results.push(ScanResult {
                    category: "IP".to_string(),
                    content: String::from_utf8_lossy(mat.as_bytes()).to_string(),
                });
            }

            // Scan for Emails
            for mat in EMAIL_REGEX.find_iter(bytes) {
                results.push(ScanResult {
                    category: "Email".to_string(),
                    content: String::from_utf8_lossy(mat.as_bytes()).to_string(),
                });
            }

            // Basic heuristic for API Keys (Google)
            // Still using a simple check on bytes
            if bytes.windows(4).any(|w| w == b"AIza") && bytes.len() > 20 {
                 results.push(ScanResult {
                    category: "API Key (Google)".to_string(),
                    content: String::from_utf8_lossy(bytes).to_string(),
                });
            }
        }

        // Deduplicate
        results.sort_by(|a, b| a.content.cmp(&b.content));
        results.dedup_by(|a, b| a.content == b.content);

        results
    }
}
