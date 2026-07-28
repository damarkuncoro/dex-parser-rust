use crate::analysis::core::visitor::InstructionVisitor;
use std::any::Any;

/// A registry to manage various analysis components.
pub struct AnalysisRegistry {
    pub visitors: Vec<Box<dyn InstructionVisitor>>,
    // Using Box<dyn Any> for analyzers since they have different Output types
    pub analyzers: Vec<Box<dyn Any + Send + Sync>>,
}

impl AnalysisRegistry {
    pub fn new() -> Self {
        Self {
            visitors: Vec::new(),
            analyzers: Vec::new(),
        }
    }

    pub fn add_visitor(&mut self, visitor: Box<dyn InstructionVisitor>) {
        self.visitors.push(visitor);
    }

    pub fn add_analyzer<T: Any + Send + Sync>(&mut self, analyzer: T) {
        self.analyzers.push(Box::new(analyzer));
    }
}

impl Default for AnalysisRegistry {
    fn default() -> Self {
        Self::new()
    }
}
