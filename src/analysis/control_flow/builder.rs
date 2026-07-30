use crate::dex::core::models::Instruction;
use crate::analysis::core::models::BasicBlock;
use crate::analysis::core::traits::DexAnalyzer;
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

        let mut offset_to_index = HashMap::with_capacity(instructions.len());
        for (i, ins) in instructions.iter().enumerate() {
            offset_to_index.insert(ins.offset, i);
        }

        for (i, ins) in instructions.iter().enumerate() {
            if let Some(target) = ins.target_offset {
                leaders.insert(target as usize);
            }

            let op_info = crate::dex::core::instructions::opcodes::OpcodeTable::get(ins.opcode);
            if op_info.can_branch || op_info.can_switch || !op_info.can_continue {
                if i + 1 < instructions.len() {
                    leaders.insert(instructions[i + 1].offset);
                }
            }
        }

        let mut sorted_leaders: Vec<usize> = leaders.into_iter().collect();
        sorted_leaders.sort_unstable();

        let mut blocks = Vec::with_capacity(sorted_leaders.len());
        let mut offset_to_block_idx = HashMap::with_capacity(sorted_leaders.len());

        let mut ins_idx = 0;
        for i in 0..sorted_leaders.len() {
            let start_offset = sorted_leaders[i];
            let end_offset = if i + 1 < sorted_leaders.len() {
                sorted_leaders[i + 1]
            } else {
                instructions.last().map(|n| n.offset + 2).unwrap_or(start_offset + 2)
            };

            let mut block_ins = Vec::new();
            while ins_idx < instructions.len() && instructions[ins_idx].offset < end_offset {
                block_ins.push(instructions[ins_idx].offset);
                ins_idx += 1;
            }

            if !block_ins.is_empty() {
                offset_to_block_idx.insert(start_offset, blocks.len());
                blocks.push(BasicBlock {
                    start_offset,
                    end_offset,
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
            let op_info = crate::dex::core::instructions::opcodes::OpcodeTable::get(last_ins.opcode);

            if op_info.can_branch {
                if let Some(target) = last_ins.target_offset {
                    if let Some(&target_block_idx) = offset_to_block_idx.get(&(target as usize)) {
                        successors.push(target_block_idx);
                    }
                }
            }

            if op_info.can_continue {
                if i + 1 < blocks.len() {
                    successors.push(i + 1);
                }
            }

            blocks[i].successors = successors;
        }

        blocks
    }

    pub fn get_reachable_offsets(instructions: &[Instruction]) -> Vec<bool> {
        let n = instructions.len();
        if n == 0 { return Vec::new(); }

        let mut reachable = vec![false; n];
        let mut stack = Vec::with_capacity(n);
        stack.push(0);

        while let Some(idx) = stack.pop() {
            if reachable[idx] { continue; }
            reachable[idx] = true;

            let ins = &instructions[idx];
            let op_info = crate::dex::core::instructions::opcodes::OpcodeTable::get(ins.opcode);

            if op_info.can_continue && idx + 1 < n {
                stack.push(idx + 1);
            }

            if op_info.can_branch {
                if let Some(target) = ins.target_offset {
                    if let Ok(target_idx) = instructions.binary_search_by_key(&target, |i| i.offset as u32) {
                        stack.push(target_idx);
                    }
                }
            }
        }

        reachable
    }
}
