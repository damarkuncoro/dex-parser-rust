use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x6e => Some(("invoke-virtual", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x6f => Some(("invoke-super", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x70 => Some(("invoke-direct", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x71 => Some(("invoke-static", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x72 => Some(("invoke-interface", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x74 => Some(("invoke-virtual/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x75 => Some(("invoke-super/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x76 => Some(("invoke-direct/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x77 => Some(("invoke-static/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x78 => Some(("invoke-interface/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0xfa => Some(("invoke-polymorphic", "{vC..vG}, meth@, proto@", 4, IndexType::Method)),
        0xfb => Some(("invoke-polymorphic/range", "{vCCCC..vNNNN}, meth@, proto@", 4, IndexType::Method)),
        0xfc => Some(("invoke-custom", "{vC..vG}, call_site@", 3, IndexType::None)),
        0xfd => Some(("invoke-custom/range", "{vCCCC..vNNNN}, call_site@", 3, IndexType::None)),
        _ => None,
    }
}
