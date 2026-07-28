use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct BasicBlock {
    pub start_offset: usize,
    pub end_offset: usize,
    pub instructions: Vec<usize>, // offsets into original list
    pub successors: Vec<usize>,  // absolute offsets of next blocks
}
