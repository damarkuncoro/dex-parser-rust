use crate::dex::core::models::Instruction;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Debug)]
pub struct BasicBlock {
    pub start_offset: usize,
    pub end_offset: usize,
    pub instructions: Vec<usize>, // offsets into original list
    pub successors: Vec<usize>,  // absolute offsets of next blocks
}

pub struct CfgBuilder;

impl CfgBuilder {
    pub fn build(instructions: &[Instruction]) -> Vec<BasicBlock> {
        if instructions.is_empty() { return Vec::new(); }

        let mut leaders = HashSet::new();
        leaders.insert(0); // First instruction

        for (i, ins) in instructions.iter().enumerate() {
            if ins.description.contains(":label_") {
                leaders.insert(ins.offset);
            }

            if is_terminator(ins.opcode) {
                if i + 1 < instructions.len() {
                    leaders.insert(instructions[i + 1].offset);
                }
            }
        }

        let mut sorted_leaders: Vec<usize> = leaders.into_iter().collect();
        sorted_leaders.sort();

        let mut blocks = Vec::new();
        for i in 0..sorted_leaders.len() {
            let start = sorted_leaders[i];
            let end = if i + 1 < sorted_leaders.len() {
                sorted_leaders[i + 1]
            } else {
                instructions.last().map(|n| n.offset + 2).unwrap_or(start) // simplified
            };

            let mut block_ins = Vec::new();
            for ins in instructions {
                if ins.offset >= start && ins.offset < end {
                    block_ins.push(ins.offset);
                }
            }

            blocks.push(BasicBlock {
                start_offset: start,
                end_offset: end,
                instructions: block_ins,
                successors: Vec::new(), // Successor calculation would be next
            });
        }

        blocks
    }
}

fn is_terminator(opcode: u8) -> bool {
    match opcode {
        0x28..=0x2a | 0x32..=0x3d | 0x0e..=0x11 | 0x27 => true,
        _ => false,
    }
}
