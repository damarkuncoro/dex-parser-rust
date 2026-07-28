pub mod models;
pub mod traits;
pub mod utils;
pub mod config;
pub mod namespace;
pub mod visitor;
pub mod registry;
pub mod pipeline;
pub mod stats_visitor;

pub use models::{
    AnalysisReport, AnalysisStats, GapAnalysis, ScanResult,
    BasicBlock, XrefMap, GlobalIntelligence, CallSite, GlobalSecuritySummary,
    AnalysisToken
};
pub use traits::{DexAnalyzer, ApkAnalyzer};
pub use utils::{Reference, ReferenceExtractor};
pub use config::{AnalysisConfig, CompiledConfig, OpcodeCategories};
pub use namespace::{NamespaceResolver, CodeScope};
pub use visitor::{InstructionVisitor, AnalysisEngine, VisitorContext};
pub use registry::AnalysisRegistry;
pub use pipeline::AnalysisPipeline;
pub use stats_visitor::{StatsVisitor, InstructionStats};
