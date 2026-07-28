use crate::dex::core::models::{Apk, Dex};
use crate::analysis::core::visitor::{AnalysisEngine, InstructionVisitor};
use crate::analysis::core::traits::ApkAnalyzer;
use crate::analysis::core::models::GlobalIntelligence;
use crate::analysis::global::GlobalAnalyzer;

/// Orchestrates the entire analysis process in a modular way.
pub struct AnalysisPipeline {
    visitors: Vec<Box<dyn InstructionVisitor>>,
}

impl AnalysisPipeline {
    pub fn new() -> Self {
        Self { visitors: Vec::new() }
    }

    pub fn add_visitor(&mut self, visitor: Box<dyn InstructionVisitor>) -> &mut Self {
        self.visitors.push(visitor);
        self
    }

    /// Runs all visitors on a single DEX file.
    pub fn run_on_dex(&mut self, dex: &Dex) {
        AnalysisEngine::walk_classes(&dex.class_defs, &mut self.visitors);
    }

    /// Runs APK-wide analysis and builds global intelligence.
    pub fn build_apk_intelligence(apk: &Apk, dex_names: &[String]) -> GlobalIntelligence {
        GlobalAnalyzer.analyze(apk, dex_names)
    }
}
