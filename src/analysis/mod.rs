pub mod core;
pub mod control_flow;
pub mod cross_ref;
pub mod forensics;
pub mod global;
pub mod tokens;

// SSOT: All data models are re-exported from core
pub use core::{
    AnalysisReport, AnalysisStats, GapAnalysis, ScanResult,
    BasicBlock, XrefMap, GlobalIntelligence, CallSite, GlobalSecuritySummary,
    AnalysisToken, RiskAssessment, RiskLevel, ScoringEngine
};

// Modular Analyzers
pub use core::{
    DexAnalyzer, ApkAnalyzer, AnalysisEngine, StatsVisitor,
    InstructionStats, VisitorContext, NamespaceResolver, CodeScope
};
pub use control_flow::CfgBuilder;
pub use cross_ref::XrefBuilder;
pub use cross_ref::builder::XrefVisitor;
pub use forensics::{
    ForensicAnalyzer, EntropyAnalyzer, StringScanner,
    BehaviorAnalyzer, BehaviorVisitor, ObfuscationVisitor, ResourceVisitor,
    engine::data_flow::DataFlowVisitor,
    engine::crypto::CryptoVisitor
};
pub use global::GlobalAnalyzer;
pub use tokens::{InstructionTokenizer, TokenizerVisitor};
