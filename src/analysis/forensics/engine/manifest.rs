use crate::dex::core::models::Manifest;
use crate::analysis::core::models::ScanResult;

pub struct ManifestAnalyzer;

impl ManifestAnalyzer {
    pub fn analyze(manifest: &Manifest) -> Vec<ScanResult> {
        let mut results = Vec::new();

        // 1. Dangerous Permission Combinations
        let has_sms_read = manifest.permissions.iter().any(|p| p.contains("READ_SMS"));
        let has_sms_receive = manifest.permissions.iter().any(|p| p.contains("RECEIVE_SMS"));
        let has_internet = manifest.permissions.iter().any(|p| p.contains("INTERNET"));

        if (has_sms_read || has_sms_receive) && has_internet {
            results.push(ScanResult {
                category: "Manifest: Dangerous Combination".to_string(),
                content: "SMS access combined with INTERNET (Potential Spyware/Exfiltration)".to_string(),
            });
        }

        // 2. Sensitive Receivers
        for receiver in &manifest.receivers {
            if receiver.contains("BOOT_COMPLETED") || receiver.contains("RECEIVE_BOOT_COMPLETED") {
                results.push(ScanResult {
                    category: "Manifest: Persistence".to_string(),
                    content: format!("Receiver starts at boot: {}", receiver),
                });
            }
        }

        // 3. Hidden Components
        if manifest.activities.is_empty() && !manifest.services.is_empty() {
             results.push(ScanResult {
                category: "Manifest: Stealth".to_string(),
                content: "No activities found but services are present (Potential background malware)".to_string(),
            });
        }

        results
    }
}
