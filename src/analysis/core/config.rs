use serde::{Serialize, Deserialize};
use regex::bytes::Regex;
use std::sync::Arc;
use crate::dex::core::instructions::opcodes::OpcodeTable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalysisConfig {
    pub forensics: ForensicConfig,
    pub scanner_rules: Vec<CustomScannerRule>,
    pub behavioral_rules: Vec<CustomBehavioralRule>,
    pub sensitive_heuristics: Vec<CustomHeuristicRule>,
    pub external_prefixes: Vec<String>,
    pub shell_commands: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForensicConfig {
    pub entropy_threshold: f64,
    pub gap_length_threshold: usize,
    pub min_string_length: usize,
    pub suspicious_string_length: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomScannerRule {
    pub category: String,
    pub pattern: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomBehavioralRule {
    pub category: String,
    pub indicator: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomHeuristicRule {
    pub category: String,
    pub pattern: String,
    pub min_len: usize,
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
                CustomBehavioralRule { category: "Dynamic Loading".to_string(), indicator: "Ldalvik/system/DexClassLoader;".to_string(), description: "Loading external DEX".to_string() },
                CustomBehavioralRule { category: "Reflection".to_string(), indicator: "Ljava/lang/reflect/Method;->invoke".to_string(), description: "Method reflection".to_string() },
                CustomBehavioralRule { category: "Crypto".to_string(), indicator: "Ljavax/crypto/Cipher;".to_string(), description: "Cryptography usage".to_string() },
                CustomBehavioralRule { category: "Native".to_string(), indicator: "Ljava/lang/System;->loadLibrary".to_string(), description: "Loading native libraries".to_string() },
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

/// Opcodes categories for CFG Analysis.
/// Now uses OpcodeTable metadata (No Hardcode Ranges!)
pub struct OpcodeCategories;

impl OpcodeCategories {
    pub fn is_branch(opcode: u8) -> bool {
        OpcodeTable::get(opcode).is_branch
    }

    pub fn is_goto(opcode: u8) -> bool {
        OpcodeTable::get(opcode).is_goto
    }

    pub fn is_switch(opcode: u8) -> bool {
        OpcodeTable::get(opcode).is_switch
    }

    pub fn is_terminator(opcode: u8) -> bool {
        OpcodeTable::get(opcode).is_terminator
    }
}
