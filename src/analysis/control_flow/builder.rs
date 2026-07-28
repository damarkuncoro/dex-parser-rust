use crate::dex::core::models::Instruction;
use crate::analysis::core::models::BasicBlock;
use crate::analysis::core::traits::DexAnalyzer;
use crate::analysis::core::config::OpcodeCategories;
use std::collections::{HashSet, HashMap};

pub struct CfgBuilder;

impl DexAnalyzer for CfgBuilder {
    type Output = Vec<BasicBlock>;
    fn analyze(&self, _dex: &crate::dex::core::models::Dex) -> Self::Output {
        Vec::new()
    }
}

impl CfgBuilder {
    pub fn build_for_method(method: &crate::dex::core::models::EncodedMethod) -> Option<Vec<BasicBlock>> {
        method.code.as_ref().map(|code| Self::build(&code.instructions))
    }

    pub fn build(instructions: &[Instruction]) -> Vec<BasicBlock> {
        if instructions.is_empty() { return Vec::new(); }

        let mut leaders = HashSet::new();
        leaders.insert(0);

        let offset_to_index: HashMap<usize, usize> = instructions.iter().enumerate()
            .map(|(i, ins)| (ins.offset, i))
            .collect();

        for (i, ins) in instructions.iter().enumerate() {
            if let Some(target) = ins.target_offset {
                leaders.insert(target as usize);
            }

            if OpcodeCategories::is_branch(ins.opcode) || OpcodeCategories::is_goto(ins.opcode) {
                if i + 1 < instructions.len() {
                    leaders.insert(instructions[i + 1].offset);
                }
            }

            if OpcodeCategories::is_switch(ins.opcode) {
                if i + 1 < instructions.len() {
                    leaders.insert(instructions[i + 1].offset);
                }
            }

            if OpcodeCategories::is_terminator(ins.opcode) {
                if i + 1 < instructions.len() {
                    leaders.insert(instructions[i + 1].offset);
                }
            }
        }

        let mut sorted_leaders: Vec<usize> = leaders.into_iter().collect();
        sorted_leaders.sort();

        let mut blocks = Vec::new();
        let mut offset_to_block_idx = HashMap::new();

        for i in 0..sorted_leaders.len() {
            let start = sorted_leaders[i];
            let end = if i + 1 < sorted_leaders.len() {
                sorted_leaders[i + 1]
            } else {
                instructions.last().map(|n| n.offset + 2).unwrap_or(start)
            };

            let block_ins: Vec<usize> = instructions.iter()
                .filter(|ins| ins.offset >= start && ins.offset < end)
                .map(|ins| ins.offset)
                .collect();

            if !block_ins.is_empty() {
                offset_to_block_idx.insert(start, blocks.len());
                blocks.push(BasicBlock {
                    start_offset: start,
                    end_offset: end,
                    instructions: block_ins,
                    successors: Vec::new(),
                });
            }
        }

        for i in 0..blocks.len() {
            let last_ins_offset = *blocks[i].instructions.last().unwrap();
            let last_ins_idx = offset_to_index[&last_ins_offset];
            let last_ins = &instructions[last_ins_idx];

            let mut successors = Vec::new();

            if OpcodeCategories::is_goto(last_ins.opcode) {
                if let Some(target) = last_ins.target_offset {
                    if let Some(&target_block_idx) = offset_to_block_idx.get(&(target as usize)) {
                        successors.push(target_block_idx);
                    }
                }
            } else if OpcodeCategories::is_branch(last_ins.opcode) {
                if let Some(target) = last_ins.target_offset {
                    if let Some(&target_block_idx) = offset_to_block_idx.get(&(target as usize)) {
                        successors.push(target_block_idx);
                    }
                }
                if i + 1 < blocks.len() {
                    successors.push(i + 1);
                }
            } else if !OpcodeCategories::is_terminator(last_ins.opcode) {
                if i + 1 < blocks.len() {
                    successors.push(i + 1);
                }
            }

            blocks[i].successors = successors;
        }

        blocks
    }
}
