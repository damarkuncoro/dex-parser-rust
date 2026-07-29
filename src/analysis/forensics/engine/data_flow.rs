use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use crate::analysis::core::models::{ScanResult, XrefMap};
use crate::analysis::core::config::AnalysisConfig;
use crate::analysis::core::utils::Reference;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct TaintEngine;

impl TaintEngine {
    pub fn propagate_inter_procedural(
        local_findings: &mut Vec<ScanResult>,
        source_returners: &HashSet<String>,
        xref_map: &XrefMap,
    ) {
        for (caller, targets) in &xref_map.method_to_methods {
            for target in targets {
                if source_returners.contains(target) {
                    local_findings.push(ScanResult {
                        category: "Data Leak: Indirect Taint".to_string(),
                        content: format!("Method {} receives tainted data from {} and may leak it.", caller, target),
                        details: None,
                    });
                }
            }
        }
    }
}

/// A visitor that performs intra-procedural data-flow analysis to detect sensitive data leaks.
pub struct DataFlowVisitor {
    config: Arc<AnalysisConfig>,
    /// Maps method signature to the set of tainted registers.
    method_taints: HashMap<String, HashSet<u16>>,
    pub findings: Vec<ScanResult>,
    pub source_returners: HashSet<String>,
}

impl DataFlowVisitor {
    pub fn new(config: Arc<AnalysisConfig>) -> Self {
        Self {
            config,
            method_taints: HashMap::new(),
            findings: Vec::new(),
            source_returners: HashSet::new(),
        }
    }

    fn is_source(&self, signature: &str) -> bool {
        self.config.taint_analysis.sources.iter().any(|s| signature.contains(s))
    }

    fn is_sink(&self, signature: &str) -> bool {
        self.config.taint_analysis.sinks.iter().any(|s| signature.contains(s))
    }
}

impl InstructionVisitor for DataFlowVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        if !self.config.taint_analysis.enabled { return; }

        let method_sig = &ctx.method.signature;
        let opcode = ctx.instruction.opcode;
        let regs = &ctx.instruction.registers;

        let mut is_source = false;
        let mut target_method = String::new();

        if let Some(Reference::Method(target)) = &ctx.reference {
            target_method = target.to_string();
            if self.is_source(target) {
                is_source = true;
            }
        }

        let mut is_leak = false;
        {
            let taints = self.method_taints.entry(method_sig.clone()).or_default();

            if is_source && !regs.is_empty() {
                taints.insert(regs[0]);
            }

            match opcode {
                0x01..=0x09 => { // move
                    if regs.len() >= 2 {
                        let dst = regs[0];
                        let src = regs[1];
                        if taints.contains(&src) {
                            taints.insert(dst);
                        } else {
                            taints.remove(&dst);
                        }
                    }
                }
                0x12..=0x1c => { // Clear on constant load
                    if !regs.is_empty() {
                        taints.remove(&regs[0]);
                    }
                }
                _ => {}
            }

            if !target_method.is_empty() && regs.iter().any(|r| taints.contains(r)) {
                is_leak = true;
            }

            // Propagate return values (Inter-procedural hint)
            if ctx.instruction.name.starts_with("return") && !regs.is_empty() {
                 if taints.contains(&regs[0]) {
                     self.source_returners.insert(method_sig.clone());
                 }
            }
        }

        if is_leak && self.is_sink(&target_method) {
            self.findings.push(ScanResult {
                category: "Data Leak: Sensitive Taint".to_string(),
                content: format!("Tainted data flows into sink: {} in method {}", target_method, method_sig),
                details: None,
            });
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.findings.extend(other.findings.clone());
            self.source_returners.extend(other.source_returners.clone());
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self::new(self.config.clone()))
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
