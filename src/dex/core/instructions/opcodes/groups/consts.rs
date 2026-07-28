use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x12 => Some(("const/4", "vA, #+B", 1, IndexType::None)),
        0x13 => Some(("const/16", "vAA, #+BBBB", 2, IndexType::None)),
        0x14 => Some(("const", "vAA, #+BBBBBBBB", 3, IndexType::None)),
        0x15 => Some(("const/high16", "vAA, #+BBBB0000", 2, IndexType::None)),
        0x16 => Some(("const-wide/16", "vAA, #+BBBB", 2, IndexType::None)),
        0x17 => Some(("const-wide/32", "vAA, #+BBBBBBBB", 3, IndexType::None)),
        0x18 => Some(("const-wide", "vAA, #+BBBBBBBBBBBBBBBB", 5, IndexType::None)),
        0x19 => Some(("const-wide/high16", "vAA, #+BBBB000000000000", 2, IndexType::None)),
        0x1a => Some(("const-string", "vAA, string@", 2, IndexType::String)),
        0x1b => Some(("const-string/jumbo", "vAAAA, string@", 3, IndexType::String)),
        0x1c => Some(("const-class", "vAA, type@", 2, IndexType::Type)),
        0xfe => Some(("const-method-handle", "vAA, method_handle@", 2, IndexType::MethodHandle)),
        0xff => Some(("const-method-type", "vAA, proto@", 2, IndexType::Proto)),
        _ => None,
    }
}
