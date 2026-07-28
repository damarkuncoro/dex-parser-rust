use crate::analysis::core::config::AnalysisConfig;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeScope {
    /// Android/Java Framework APIs (e.g., Landroid/*, Ljava/*)
    System,
    /// 3rd Party Libraries (e.g., Lcom/facebook/*, Lorg/apache/*)
    ThirdParty,
    /// The actual application code
    User,
}

pub struct NamespaceResolver {
    config: Arc<AnalysisConfig>,
}

impl NamespaceResolver {
    pub fn new(config: Arc<AnalysisConfig>) -> Self {
        Self { config }
    }

    /// Resolves the scope of a class or method signature.
    pub fn resolve(&self, signature: &str) -> CodeScope {
        // 1. Check for System API prefixes
        if self.config.external_prefixes.iter().any(|p| signature.starts_with(p)) {
            return CodeScope::System;
        }

        // 2. Identify common Third Party patterns (could be made configurable too)
        let third_party_prefixes = ["Lcom/google/", "Landroidx/", "Lorg/", "Lkotlinx/"];
        if third_party_prefixes.iter().any(|&p| signature.starts_with(p)) {
            return CodeScope::ThirdParty;
        }

        // 3. Everything else is likely User code
        CodeScope::User
    }

    pub fn is_system(&self, signature: &str) -> bool {
        self.resolve(signature) == CodeScope::System
    }

    pub fn is_user(&self, signature: &str) -> bool {
        self.resolve(signature) == CodeScope::User
    }
}
