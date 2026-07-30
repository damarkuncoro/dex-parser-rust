use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use crate::analysis::core::models::{ScanResult, forensic::CryptoDetails};
use crate::analysis::core::utils::Reference;
use std::collections::HashMap;

pub struct CryptoVisitor {
    /// Tracks the last known string value in each register per method.
    register_strings: HashMap<String, HashMap<u16, String>>,
    pub findings: Vec<ScanResult>,
}

impl CryptoVisitor {
    pub fn new() -> Self {
        Self {
            register_strings: HashMap::new(),
            findings: Vec::new(),
        }
    }

    fn analyze_transformation(&self, transformation: &str, key_used: Option<String>) -> CryptoDetails {
        let parts: Vec<&str> = transformation.split('/').collect();
        let algo = parts.get(0).unwrap_or(&"Unknown").to_string();
        let mode = parts.get(1).unwrap_or(&"None").to_string();
        let padding = parts.get(2).unwrap_or(&"None").to_string();

        let mut risk = "LOW";
        let mut reason = "Standard implementation".to_string();

        if mode == "ECB" {
            risk = "HIGH";
            reason = "ECB mode is insecure because it does not provide serious data confidentiality.".to_string();
        } else if algo == "DES" {
            risk = "HIGH";
            reason = "DES algorithm is deprecated and weak against brute force.".to_string();
        } else if padding == "NoPadding" && mode == "CBC" {
            risk = "MEDIUM";
            reason = "Using NoPadding with CBC can be risky if not handled carefully.".to_string();
        }

        if key_used.is_some() {
            reason += " (Hardcoded key detected!)";
            risk = "CRITICAL";
        }

        CryptoDetails {
            algorithm: algo,
            mode,
            padding,
            risk: risk.to_string(),
            reason,
        }
    }
}

impl InstructionVisitor for CryptoVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        let method_sig = &ctx.method.signature;
        let regs = &ctx.instruction.registers;
        let strings = self.register_strings.entry(method_sig.clone()).or_default();

        // 1. Track constants (const-string)
        if let Some(Reference::String(val)) = &ctx.reference {
            if !regs.is_empty() {
                strings.insert(regs[0], val.to_string());
            }
        }

        // 2. Detect SecretKeySpec initialization
        let mut found_key = None;
        if let Some(Reference::Method(target)) = &ctx.reference {
            if target.contains("SecretKeySpec;-><init>") {
                 if let Some(arg_reg) = regs.get(0) {
                     if let Some(k) = strings.get(arg_reg) {
                         found_key = Some(k.clone());
                     }
                 }
            }
        }

        // 3. Detect Cipher.getInstance
        let mut found_transformation = None;
        if let Some(Reference::Method(target)) = &ctx.reference {
            if target.contains("Ljavax/crypto/Cipher;->getInstance") {
                if let Some(arg_reg) = regs.get(0) {
                    if let Some(transformation) = strings.get(arg_reg) {
                        found_transformation = Some(transformation.clone());
                    }
                }
            }
        }

        if let Some(transformation) = found_transformation {
            let details = self.analyze_transformation(&transformation, found_key);
            self.findings.push(ScanResult {
                category: "Cryptography".to_string(),
                content: format!("Algorithm: {}", transformation),
                details: Some(details),
            });
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.findings.extend(other.findings.clone());
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self::new())
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
