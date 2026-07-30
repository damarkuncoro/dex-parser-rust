pub mod forensic;
pub mod rules;
pub mod taint;
pub mod categories;

use serde::{Serialize, Deserialize};
use regex::bytes::Regex;
use std::sync::Arc;
pub use forensic::ForensicConfig;
pub use rules::{ManifestRule, ScoringRule, IntelligenceRule, CustomScannerRule, CustomBehavioralRule, CustomHeuristicRule};
pub use taint::TaintConfig;
pub use categories::OpcodeCategories;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalysisConfig {
    pub forensics: ForensicConfig,
    pub scanner_rules: Vec<CustomScannerRule>,
    pub behavioral_rules: Vec<CustomBehavioralRule>,
    pub sensitive_heuristics: Vec<CustomHeuristicRule>,
    pub external_prefixes: Vec<String>,
    pub shell_commands: Vec<String>,
    pub taint_analysis: TaintConfig,
    pub scoring_rules: Vec<ScoringRule>,
    pub intelligence_rules: Vec<IntelligenceRule>,
    pub manifest_rules: Vec<ManifestRule>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            forensics: ForensicConfig {
                entropy_threshold: 6.5,
                gap_length_threshold: 100,
                min_string_length: 4,
                suspicious_string_length: 512,
            },
            scanner_rules: vec![
                CustomScannerRule { category: "URL".to_string(), pattern: r"https?://[^\s/$.?#].[^\s]*".to_string() },
                CustomScannerRule { category: "IP".to_string(), pattern: r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string() },
                CustomScannerRule { category: "Email".to_string(), pattern: r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string() },
                CustomScannerRule { category: "Shell Command".to_string(), pattern: r"\b(chmod|chown|rm -rf|sh -c|su|mount -o)\b".to_string() },
            ],
            behavioral_rules: vec![
                CustomBehavioralRule {
                    category: "Dynamic Loading".to_string(),
                    indicator: "Ldalvik/system/DexClassLoader;".to_string(),
                    description: "Loading external DEX".to_string(),
                    mitre_id: Some("T1071".to_string()),
                    required_permission: None
                },
                CustomBehavioralRule {
                    category: "Reflection".to_string(),
                    indicator: "Ljava/lang/reflect/Method;->invoke".to_string(),
                    description: "Method reflection".to_string(),
                    mitre_id: Some("T1129".to_string()),
                    required_permission: None
                },
                CustomBehavioralRule {
                    category: "Crypto".to_string(),
                    indicator: "Ljavax/crypto/Cipher;".to_string(),
                    description: "Cryptography usage".to_string(),
                    mitre_id: Some("T1573".to_string()),
                    required_permission: None
                },
                CustomBehavioralRule {
                    category: "Sms".to_string(),
                    indicator: "Landroid/telephony/SmsManager;->sendTextMessage".to_string(),
                    description: "Sending SMS".to_string(),
                    mitre_id: Some("T1589".to_string()),
                    required_permission: Some("SEND_SMS".to_string())
                },
                CustomBehavioralRule {
                    category: "Native".to_string(),
                    indicator: "Ljava/lang/System;->loadLibrary".to_string(),
                    description: "Loading native libraries".to_string(),
                    mitre_id: Some("T1129".to_string()),
                    required_permission: None
                },
            ],
            sensitive_heuristics: vec![
                CustomHeuristicRule { category: "API Key (Google)".to_string(), pattern: "AIza".to_string(), min_len: 20 },
            ],
            external_prefixes: vec![
                "Landroid/".to_string(), "Ljava/".to_string(), "Ljavax/".to_string(),
                "Lkotlin/".to_string(), "Lcom/android/".to_string(), "Lcom/google/".to_string()
            ],
            shell_commands: vec![
                "chmod ".to_string(), "chown ".to_string(), "rm -rf ".to_string(),
                "su ".to_string(), "mount ".to_string(), "sh -c ".to_string()
            ],
            taint_analysis: TaintConfig {
                enabled: true,
                sources: vec![
                    "Landroid/telephony/TelephonyManager;->getDeviceId".to_string(),
                    "Landroid/location/Location;->getLatitude".to_string(),
                    "Landroid/content/ContentResolver;->query".to_string(),
                ],
                sinks: vec![
                    "Ljava/net/URL;->openConnection".to_string(),
                    "Landroid/telephony/SmsManager;->sendTextMessage".to_string(),
                    "Ljava/lang/Runtime;->exec".to_string(),
                ],
            },
            scoring_rules: vec![
                ScoringRule { category_pattern: "Manifest: Dangerous Combination".to_string(), points: 4.0, justification_prefix: "CRITICAL".to_string() },
                ScoringRule { category_pattern: "Manifest: Persistence".to_string(), points: 1.5, justification_prefix: "Suspicious".to_string() },
                ScoringRule { category_pattern: "Manifest: Stealth".to_string(), points: 2.0, justification_prefix: "Suspicious".to_string() },
                ScoringRule { category_pattern: "Data Leak: Sensitive Taint".to_string(), points: 5.0, justification_prefix: "CRITICAL LEAK".to_string() },
                ScoringRule { category_pattern: "Behavior: Crypto".to_string(), points: 1.0, justification_prefix: "Notice".to_string() },
                ScoringRule { category_pattern: "Behavior: Reflection".to_string(), points: 0.8, justification_prefix: "Notice".to_string() },
                ScoringRule { category_pattern: "Behavior: Native".to_string(), points: 1.2, justification_prefix: "Notice".to_string() },
            ],
            intelligence_rules: vec![
                IntelligenceRule {
                    name: "Potential Spyware/Exfiltration".to_string(),
                    description: "App accesses sensitive user data and shows pathways to transmit it over the network.".to_string(),
                    severity: crate::analysis::core::models::RiskLevel::High,
                    mitre_id: Some("T1512".to_string()),
                    required_permissions: vec!["ACCESS_FINE_LOCATION".to_string(), "READ_SMS".to_string()],
                    required_behaviors: vec!["URL".to_string()],
                    requires_taint: true,
                },
                IntelligenceRule {
                    name: "Ransomware Pattern".to_string(),
                    description: "Combination of high cryptography usage, boot persistence, and file system access.".to_string(),
                    severity: crate::analysis::core::models::RiskLevel::Critical,
                    mitre_id: Some("T1486".to_string()),
                    required_permissions: vec!["RECEIVE_BOOT_COMPLETED".to_string(), "WRITE_EXTERNAL_STORAGE".to_string()],
                    required_behaviors: vec!["Crypto".to_string()],
                    requires_taint: false,
                },
                IntelligenceRule {
                    name: "Evasive Execution".to_string(),
                    description: "Uses multiple dynamic code invocation techniques to bypass static analysis.".to_string(),
                    severity: crate::analysis::core::models::RiskLevel::Medium,
                    mitre_id: Some("T1027".to_string()),
                    required_permissions: vec![],
                    required_behaviors: vec!["Reflection".to_string(), "Dynamic Loading".to_string()],
                    requires_taint: false,
                },
                IntelligenceRule {
                    name: "Privilege Escalation Attempt".to_string(),
                    description: "Attempts to gain root access using 'su' commands.".to_string(),
                    severity: crate::analysis::core::models::RiskLevel::High,
                    mitre_id: Some("T1548".to_string()),
                    required_permissions: vec![],
                    required_behaviors: vec!["Shell Command".to_string()],
                    requires_taint: false,
                },
                IntelligenceRule {
                    name: "Cryptographic Operations".to_string(),
                    description: "The application performs data encryption or decryption.".to_string(),
                    severity: crate::analysis::core::models::RiskLevel::Low,
                    mitre_id: Some("T1573".to_string()),
                    required_permissions: vec![],
                    required_behaviors: vec!["Crypto".to_string()],
                    requires_taint: false,
                },
            ],
            manifest_rules: vec![
                ManifestRule {
                    category: "Manifest: Dangerous Combination".to_string(),
                    description: "SMS access combined with INTERNET (Potential Spyware/Exfiltration)".to_string(),
                    required_permissions: vec!["READ_SMS".to_string(), "INTERNET".to_string()],
                    required_actions: vec![],
                    must_have_no_activities: false,
                },
                ManifestRule {
                    category: "Manifest: Persistence".to_string(),
                    description: "Component starts at boot".to_string(),
                    required_permissions: vec![],
                    required_actions: vec!["BOOT_COMPLETED".to_string(), "RECEIVE_BOOT_COMPLETED".to_string()],
                    must_have_no_activities: false,
                },
                ManifestRule {
                    category: "Manifest: Stealth".to_string(),
                    description: "No activities found but services are present (Potential background malware)".to_string(),
                    required_permissions: vec![],
                    required_actions: vec![],
                    must_have_no_activities: true,
                },
            ],
        }
    }
}

pub struct CompiledConfig {
    pub config: AnalysisConfig,
    pub scanner_regex: Vec<(String, Regex)>,
}

impl CompiledConfig {
    pub fn compile(config: AnalysisConfig) -> Result<Arc<Self>, String> {
        let mut scanner_regex = Vec::new();
        for r in &config.scanner_rules {
            let re = Regex::new(&r.pattern)
                .map_err(|e| format!("Invalid regex pattern for category '{}': {}", r.category, e))?;
            scanner_regex.push((r.category.clone(), re));
        }

        Ok(Arc::new(Self {
            config,
            scanner_regex,
        }))
    }
}
