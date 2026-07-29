use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use crate::analysis::core::utils::Reference;
use crate::analysis::core::config::OpcodeCategories;
use std::sync::Arc;
use parking_lot::Mutex;

use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct InstructionStats {
    pub call_count: usize,
    pub jump_count: usize,
    pub string_count: usize,
    pub type_count: usize,
    pub field_count: usize,
    pub unknown_opcodes_count: usize,
    pub spec_violation_count: usize,
    pub unknown_opcodes_distribution: HashMap<u8, usize>,
    pub consecutive_nop_count: usize,
    pub max_consecutive_nops: usize,
    pub dead_code_count: usize,
}

pub struct StatsVisitor {
    pub stats: Arc<Mutex<InstructionStats>>,
    current_nop_run: usize,
}

impl StatsVisitor {
    pub fn new(stats: Arc<Mutex<InstructionStats>>) -> Self {
        Self { stats, current_nop_run: 0 }
    }
}

impl InstructionVisitor for StatsVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        let mut stats = self.stats.lock();

        let op_info = crate::dex::core::instructions::opcodes::OpcodeTable::get(ctx.instruction.opcode);

        if op_info.is_unused_spec {
            stats.spec_violation_count += 1;
            *stats.unknown_opcodes_distribution.entry(ctx.instruction.opcode).or_default() += 1;
        }

        if ctx.instruction.name.starts_with("op_") {
            stats.unknown_opcodes_count += 1;
            *stats.unknown_opcodes_distribution.entry(ctx.instruction.opcode).or_default() += 1;
        }

        // NOP Pattern Analysis (Opcode 0x00)
        if ctx.instruction.opcode == 0x00 {
            self.current_nop_run += 1;
            stats.consecutive_nop_count += 1;
            if self.current_nop_run > stats.max_consecutive_nops {
                stats.max_consecutive_nops = self.current_nop_run;
            }
        } else {
            self.current_nop_run = 0;
        }

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
        Box::new(Self { stats: self.stats.clone(), current_nop_run: 0 })
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
