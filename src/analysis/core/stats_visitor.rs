use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use crate::analysis::core::utils::Reference;
use crate::analysis::core::config::OpcodeCategories;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Default, Clone, Debug)]
pub struct InstructionStats {
    pub call_count: usize,
    pub jump_count: usize,
    pub string_count: usize,
    pub type_count: usize,
    pub field_count: usize,
}

pub struct StatsVisitor {
    pub stats: Arc<Mutex<InstructionStats>>,
}

impl StatsVisitor {
    pub fn new(stats: Arc<Mutex<InstructionStats>>) -> Self {
        Self { stats }
    }
}

impl InstructionVisitor for StatsVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        let mut stats = self.stats.lock();

        if OpcodeCategories::is_goto(ctx.instruction.opcode) || OpcodeCategories::is_branch(ctx.instruction.opcode) {
            stats.jump_count += 1;
        }

        if let Some(r) = &ctx.reference {
            match r {
                Reference::Method(_) => stats.call_count += 1,
                Reference::Field(_) => stats.field_count += 1,
                Reference::String(_) => stats.string_count += 1,
                Reference::Type(_) => stats.type_count += 1,
                Reference::CallSite(_) | Reference::MethodHandle(_) | Reference::Proto(_) => {
                    // Count advanced references if needed
                }
            }
        }
    }

    fn merge(&mut self, _other: Box<dyn InstructionVisitor>) {
        // Since we use Arc<Mutex>, data is already merged in the shared object
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self { stats: self.stats.clone() })
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
