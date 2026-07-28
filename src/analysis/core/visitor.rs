use crate::dex::core::models::{Class, EncodedMethod, Instruction};
use crate::analysis::core::utils::{Reference, ReferenceExtractor};
use rayon::prelude::*;

/// Context provided to each visitor during the walk.
pub struct VisitorContext<'a> {
    pub class: &'a Class<'a>,
    pub method: &'a EncodedMethod<'a>,
    pub instruction: &'a Instruction,
    pub reference: Option<Reference<'a>>,
}

/// Trait for components that want to inspect every instruction in a DEX.
pub trait InstructionVisitor: Send + Sync {
    /// Called once for every class found.
    fn visit_class(&mut self, _class: &Class) {}

    /// Called once for every method found.
    fn visit_method(&mut self, _class: &Class, _method: &EncodedMethod) {}

    /// Called for every instruction found during the walk.
    fn visit_instruction(&mut self, ctx: &VisitorContext);

    /// For parallel execution: merge another visitor of the same type into this one.
    fn merge(&mut self, other: Box<dyn InstructionVisitor>);

    /// Create a new instance of the same type for parallel processing.
    fn clone_factory(&self) -> Box<dyn InstructionVisitor>;

    /// Support for downcasting if needed.
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct AnalysisEngine;

impl AnalysisEngine {
    /// Orchestrates a single-pass iteration over all instructions in parallel.
    pub fn walk_classes_parallel(classes: &[Class], visitors: &[Box<dyn InstructionVisitor>]) -> Vec<Box<dyn InstructionVisitor>> {
        classes.par_iter()
            .map(|class| {
                let mut local_visitors: Vec<Box<dyn InstructionVisitor>> = visitors.iter()
                    .map(|v| v.clone_factory())
                    .collect();

                for visitor in local_visitors.iter_mut() {
                    visitor.visit_class(class);
                }

                let all_methods = class.direct_methods.iter().chain(class.virtual_methods.iter());
                for method in all_methods {
                    for visitor in local_visitors.iter_mut() {
                        visitor.visit_method(class, method);
                    }

                    if let Some(code) = &method.code {
                        for ins in &code.instructions {
                            let reference = ReferenceExtractor::extract(ins);
                            let ctx = VisitorContext {
                                class,
                                method,
                                instruction: ins,
                                reference,
                            };

                            for visitor in local_visitors.iter_mut() {
                                visitor.visit_instruction(&ctx);
                            }
                        }
                    }
                }
                local_visitors
            })
            .reduce(
                || visitors.iter().map(|v| v.clone_factory()).collect(),
                |mut a, b| {
                    for (v_a, v_b) in a.iter_mut().zip(b.into_iter()) {
                        v_a.merge(v_b);
                    }
                    a
                }
            )
    }

    /// Orchestrates a single-pass iteration over all instructions sequentially.
    pub fn walk_classes(classes: &[Class], visitors: &mut [Box<dyn InstructionVisitor>]) {
        for class in classes {
            for visitor in visitors.iter_mut() {
                visitor.visit_class(class);
            }

            let all_methods = class.direct_methods.iter().chain(class.virtual_methods.iter());
            for method in all_methods {
                for visitor in visitors.iter_mut() {
                    visitor.visit_method(class, method);
                }

                if let Some(code) = &method.code {
                    for ins in &code.instructions {
                        let reference = ReferenceExtractor::extract(ins);
                        let ctx = VisitorContext {
                            class,
                            method,
                            instruction: ins,
                            reference,
                        };

                        for visitor in visitors.iter_mut() {
                            visitor.visit_instruction(&ctx);
                        }
                    }
                }
            }
        }
    }
}
