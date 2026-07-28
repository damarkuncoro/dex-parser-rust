use crate::analysis::core::models::ScanResult;
use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use crate::analysis::core::config::AnalysisConfig;
use crate::analysis::forensics::rules::BehaviorScanner;
use std::sync::Arc;

pub struct BehaviorVisitor {
    config: Arc<AnalysisConfig>,
    pub results: Vec<ScanResult>,
}

impl BehaviorVisitor {
    pub fn new(config: Arc<AnalysisConfig>) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    pub fn consume_results(mut self) -> Vec<ScanResult> {
        self.results.sort_by(|a, b| a.content.cmp(&b.content));
        self.results.dedup_by(|a, b| a.content == b.content);
        self.results
    }
}

impl InstructionVisitor for BehaviorVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        if let Some(r) = &ctx.reference {
            if let Some(found) = BehaviorScanner::check_reference(r, &self.config) {
                self.results.push(found);
            }
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.results.extend(other.results.clone());
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self {
            config: self.config.clone(),
            results: Vec::new(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
