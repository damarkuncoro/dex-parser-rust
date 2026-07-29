use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicBlock {
    pub start_offset: usize,
    pub end_offset: usize,
    pub instructions: Vec<usize>,
    pub successors: Vec<usize>,
}
