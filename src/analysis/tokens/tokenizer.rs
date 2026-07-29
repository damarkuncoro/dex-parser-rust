use crate::dex::core::models::{Class, Instruction};
use crate::analysis::core::models::AnalysisToken;
use crate::analysis::core::utils::{Reference, ReferenceExtractor};
use crate::analysis::core::config::AnalysisConfig;
use crate::analysis::core::namespace::{NamespaceResolver, CodeScope};
use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use std::collections::HashMap;
use std::sync::Arc;
use rayon::prelude::*;

pub struct TokenizerVisitor {
    config: Arc<AnalysisConfig>,
    resolver: NamespaceResolver,
    pub results: HashMap<String, Vec<AnalysisToken>>,
}

impl TokenizerVisitor {
    pub fn new(config: Arc<AnalysisConfig>) -> Self {
        Self {
            config: config.clone(),
            resolver: NamespaceResolver::new(config),
            results: HashMap::new(),
        }
    }
}

impl InstructionVisitor for TokenizerVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        if let Some(r) = &ctx.reference {
            if let Some(token) = InstructionTokenizer::tokenize_reference(r, &self.config, &self.resolver) {
                self.results.entry(ctx.method.signature.clone()).or_default().push(token);
            }
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            for (k, v) in &other.results {
                self.results.entry(k.clone()).or_default().extend(v.clone());
            }
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self {
            config: self.config.clone(),
            resolver: NamespaceResolver::new(self.config.clone()),
            results: HashMap::new(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

fn is_shell_command(s: &str, config: &AnalysisConfig) -> bool {
    config.shell_commands.iter().any(|c| s.contains(c))
}

// InstructionTokenizer for high-speed parallel processing
pub struct InstructionTokenizer;

impl InstructionTokenizer {
    pub fn tokenize_classes(classes: &[Class], config: Arc<AnalysisConfig>) -> HashMap<String, Vec<AnalysisToken>> {
        let resolver = NamespaceResolver::new(config.clone());

        classes.par_iter()
            .flat_map(|class| {
                let methods = class.direct_methods.iter().chain(class.virtual_methods.iter());
                methods.par_bridge().filter_map(|method| {
                    method.code.as_ref().map(|code| {
                        (method.signature.clone(), Self::tokenize_instructions(&code.instructions, &config, &resolver))
                    })
                })
            })
            .collect()
    }

    pub fn tokenize_instructions(
        instructions: &[Instruction],
        config: &AnalysisConfig,
        resolver: &NamespaceResolver
    ) -> Vec<AnalysisToken> {
        instructions.iter()
            .filter_map(|ins| {
                let reference = ReferenceExtractor::extract(ins);
                reference.and_then(|r| Self::tokenize_reference(&r, config, resolver))
            })
            .collect()
    }

    pub fn tokenize_reference(
        reference: &Reference,
        config: &AnalysisConfig,
        resolver: &NamespaceResolver
    ) -> Option<AnalysisToken> {
        match reference {
            Reference::Method(target) => {
                let scope = resolver.resolve(target);
                if scope == CodeScope::System {
                    for rule in &config.behavioral_rules {
                        if target.contains(&rule.indicator) {
                            match rule.category.as_str() {
                                "Crypto" => return Some(AnalysisToken::CryptoOp(target.to_string())),
                                "Dynamic Loading" => return Some(AnalysisToken::DynamicLoad),
                                "Reflection" => return Some(AnalysisToken::Reflection),
                                "Native" => return Some(AnalysisToken::NativeLoad(target.to_string())),
                                _ => {}
                            }
                        }
                    }
                    Some(AnalysisToken::ExternalCall(target.to_string()))
                } else {
                    Some(AnalysisToken::InternalCall(target.to_string()))
                }
            }
            Reference::String(target) => {
                if is_shell_command(target, config) {
                    Some(AnalysisToken::SystemCommand(target.to_string()))
                } else if target.len() > 10 {
                    Some(AnalysisToken::StringUsage(target.to_string()))
                } else {
                    None
                }
            }
            Reference::Type(target) => {
                let scope = resolver.resolve(target);
                if scope == CodeScope::System {
                    Some(AnalysisToken::ExternalCall(format!("Type:{}", target)))
                } else {
                    Some(AnalysisToken::InternalCall(format!("Type:{}", target)))
                }
            }
            _ => None,
        }
    }
}
