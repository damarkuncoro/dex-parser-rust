use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AnalysisToken {
    ExternalCall(String),
    InternalCall(String),
    StringUsage(String),
    CryptoOp(String),
    NativeLoad(String),
    DynamicLoad,
    Reflection,
    SystemCommand(String),
}
