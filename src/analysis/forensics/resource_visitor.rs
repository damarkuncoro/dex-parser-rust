use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};
use std::collections::HashMap;

pub struct ResourceVisitor {
    pub found_ids: HashMap<u32, String>,
}

impl ResourceVisitor {
    pub fn new() -> Self {
        Self { found_ids: HashMap::new() }
    }
}

impl InstructionVisitor for ResourceVisitor {
    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        for &imm in &ctx.instruction.immediates {
            let val = imm as u32;
            // Check if it looks like a typical app resource ID (0x7f......)
            if (val >> 24) == 0x7f {
                self.found_ids.insert(val, format!("0x{:08x}", val));
            }
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            for (k, v) in &other.found_ids {
                self.found_ids.insert(*k, v.clone());
            }
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self::new())
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
