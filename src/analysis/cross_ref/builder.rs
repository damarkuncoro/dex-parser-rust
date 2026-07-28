use crate::dex::core::models::{Dex, Class};
use crate::analysis::core::traits::DexAnalyzer;
use crate::analysis::core::utils::{Reference, ReferenceExtractor};
use crate::analysis::core::models::XrefMap;
use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use rayon::prelude::*;

pub struct XrefBuilder;

impl DexAnalyzer for XrefBuilder {
    type Output = XrefMap;
    fn analyze(&self, dex: &Dex) -> Self::Output {
        Self::build(&dex.class_defs)
    }
}

pub struct XrefVisitor {
    pub map: XrefMap,
}

impl XrefVisitor {
    pub fn new() -> Self {
        Self { map: XrefMap::default() }
    }
}

impl InstructionVisitor for XrefVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        if let Some(reference) = &ctx.reference {
            let caller_sig = &ctx.method.signature;
            match reference {
                Reference::Method(target) => {
                    self.map.method_to_methods.entry(caller_sig.clone())
                        .or_default().push(target.to_string());
                }
                Reference::Field(target) => {
                    self.map.method_to_fields.entry(caller_sig.clone())
                        .or_default().push(target.to_string());
                }
                Reference::String(target) => {
                    self.map.method_to_strings.entry(caller_sig.clone())
                        .or_default().push(target.to_string());
                }
                Reference::Type(target) => {
                    self.map.method_to_types.entry(caller_sig.clone())
                        .or_default().push(target.to_string());
                }
                _ => {}
            }
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.map.merge(other.map.clone());
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self::new())
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

impl XrefBuilder {
    pub fn build(classes: &[Class]) -> XrefMap {
        let mut map = classes.par_iter()
            .map(|class| {
                let mut local_map = XrefMap::default();
                let methods = class.direct_methods.iter().chain(class.virtual_methods.iter());

                for method in methods {
                    if let Some(code) = &method.code {
                        let caller_sig = &method.signature;

                        for ins in &code.instructions {
                            if let Some(reference) = ReferenceExtractor::extract(ins) {
                                match reference {
                                    Reference::Method(target) => {
                                        local_map.method_to_methods.entry(caller_sig.clone())
                                            .or_default().push(target.to_string());
                                    }
                                    Reference::Field(target) => {
                                        local_map.method_to_fields.entry(caller_sig.clone())
                                            .or_default().push(target.to_string());
                                    }
                                    Reference::String(target) => {
                                        local_map.method_to_strings.entry(caller_sig.clone())
                                            .or_default().push(target.to_string());
                                    }
                                    Reference::Type(target) => {
                                        local_map.method_to_types.entry(caller_sig.clone())
                                            .or_default().push(target.to_string());
                                    }
                                    _ => {} // Other reference types ignored for Xref for now
                                }
                            }
                        }
                    }
                }
                local_map
            })
            .reduce(XrefMap::default, |mut a, b| {
                a.merge(b);
                a
            });

        map.deduplicate();
        map
    }
}
