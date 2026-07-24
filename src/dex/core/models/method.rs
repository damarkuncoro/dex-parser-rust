use serde::{Serialize};
use super::class::Code;

#[derive(Serialize)]
pub struct EncodedMethod<'a> {
    pub name: String,
    pub signature: String,
    pub access_flags: u32,
    pub access_flags_text: String,
    pub code_off: u32,
    pub code: Option<Code<'a>>,
}
