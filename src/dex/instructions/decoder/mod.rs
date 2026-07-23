use crate::dex::instructions::opcodes::{IndexType, OpcodeTable};
use crate::dex::models::Instruction;
use crate::dex::parsers::traits::DexResolver;
use scroll::{Endian, Pread};
use std::marker::PhantomData;

/// Decodes raw Dalvik bytecode into high-level `Instruction` structures.
pub struct InstructionDecoder<'res, 'a, R: DexResolver<'a>> {
    resolver: &'res R,
    _marker: PhantomData<&'a ()>,
}

impl<'res, 'a, R: DexResolver<'a>> InstructionDecoder<'res, 'a, R> {
    pub fn new(resolver: &'res R) -> Self {
        Self {
            resolver,
            _marker: PhantomData,
        }
    }

    pub fn decode(
        &self,
        buffer: &[u8],
        pc: usize,
        curr: usize,
        endian: Endian,
    ) -> (Instruction, usize) {
        let opcode_byte: u8 = buffer.pread_with(pc, endian).unwrap_or(0);
        let info = OpcodeTable::get(opcode_byte);

        let mut description = info.format.clone();
        let mut units = Vec::new();
        for i in 0..info.length {
            let unit: u16 = buffer.pread_with(pc + (i * 2), endian).unwrap_or(0);
            units.push(unit);
        }

        let current_instr_byte_addr = pc - curr;

        if !units.is_empty() {
            let op_unit = units[0];

            // 1. Substitute Special Ranges & Multi-Register Formats (Order: Longest first)
            if description.contains("{vC..vG}") {
                let count = (op_unit >> 12) & 0xf;
                let g = (op_unit >> 8) & 0xf;
                let regs_unit = units.get(2).cloned().unwrap_or(0);
                let c = regs_unit & 0xf;
                let d = (regs_unit >> 4) & 0xf;
                let e = (regs_unit >> 8) & 0xf;
                let f = (regs_unit >> 12) & 0xf;

                let mut regs = Vec::new();
                if count > 0 { regs.push(format!("v{}", c)); }
                if count > 1 { regs.push(format!("v{}", d)); }
                if count > 2 { regs.push(format!("v{}", e)); }
                if count > 3 { regs.push(format!("v{}", f)); }
                if count > 4 { regs.push(format!("v{}", g)); }
                description = description.replace("{vC..vG}", &format!("{{{}}}", regs.join(", ")));
            } else if description.contains("{vCCCC..vNNNN}") {
                let count = (op_unit >> 8) & 0xff;
                let start = units.get(2).cloned().unwrap_or(0);
                description = description.replace("{vCCCC..vNNNN}", &format!("{{v{} .. v{}}}", start, start + count as u16 - 1));
            }

            // 2. Substitute Immediates & Constants (Longest patterns first)
            if description.contains("#+BBBBBBBB") {
                let b: u32 = buffer.pread_with(pc + 2, endian).unwrap_or(0);
                description = description.replace("#+BBBBBBBB", &format!("#0x{:08x}", b));
            } else if description.contains("#+BBBB000000000000") {
                let b = units.get(1).cloned().unwrap_or(0) as u64;
                description = description.replace("#+BBBB000000000000", &format!("#0x{:016x}", b << 48));
            } else if description.contains("#+BBBB0000") {
                let b = units.get(1).cloned().unwrap_or(0) as u32;
                description = description.replace("#+BBBB0000", &format!("#0x{:08x}", b << 16));
            } else if description.contains("#+BBBB") {
                let b = units.get(1).cloned().unwrap_or(0) as i16;
                description = description.replace("#+BBBB", &format!("#{:+} (0x{:04x})", b, b as u16));
            } else if description.contains("#+CC") {
                let cc = (units.get(1).cloned().unwrap_or(0) >> 8) as i8;
                description = description.replace("#+CC", &format!("#{}", cc));
            } else if description.contains("#+B") {
                let b = (op_unit >> 12) as i8;
                description = description.replace("#+B", &format!("#{:+} (0x{:x})", b, b as u8 & 0xf));
            }

            // 3. Substitute Branch Offsets (Absolute labels)
            if description.contains("+CCCC") {
                let off = units.get(1).cloned().unwrap_or(0) as i16;
                description = description.replace("+CCCC", &format!(":label_{:04x}", (current_instr_byte_addr as i32 + (off as i32 * 2)) as u32));
            } else if description.contains("+BBBB") {
                let off = units.get(1).cloned().unwrap_or(0) as i16;
                description = description.replace("+BBBB", &format!(":label_{:04x}", (current_instr_byte_addr as i32 + (off as i32 * 2)) as u32));
            } else if description.contains("+AA") {
                let off = ((op_unit >> 8) & 0xff) as i8;
                description = description.replace("+AA", &format!(":label_{:04x}", (current_instr_byte_addr as i32 + (off as i32 * 2)) as u32));
            }

            // 4. Substitute Registers (Order: Longest first)
            description = description.replace("vAAAA", &format!("v{}", units.get(1).cloned().unwrap_or(0)));

            // Format 23x (vAA, vBB, vCC)
            if description.contains("vBB") && description.contains("vCC") {
                let v_unit = units.get(1).cloned().unwrap_or(0);
                description = description.replace("vBB", &format!("v{}", v_unit & 0xff));
                description = description.replace("vCC", &format!("v{}", (v_unit >> 8) & 0xff));
            }

            // Format 22b (vAA, vBB, #+CC)
            if description.contains("vAA") && description.contains("vBB") {
                 description = description.replace("vAA", &format!("v{}", (op_unit >> 8) & 0xff));
                 description = description.replace("vBB", &format!("v{}", units.get(1).cloned().unwrap_or(0) & 0xff));
            }

            description = description.replace("vAA", &format!("v{}", (op_unit >> 8) & 0xff));
            description = description.replace("vA", &format!("v{}", (op_unit >> 8) & 0xf));
            description = description.replace("vB", &format!("v{}", (op_unit >> 12) & 0xf));

            // 5. Substitute Cross-References
            if info.index_type != IndexType::None {
                let index: u32 = if opcode_byte == 0x1b { // const-string/jumbo
                    buffer.pread_with(pc + 2, endian).unwrap_or(0)
                } else {
                    units.get(1).cloned().unwrap_or(0) as u32
                };

                let resolved = match info.index_type {
                    IndexType::String => self.resolver.resolve_string(index)
                        .map(|s| format!("\"{}\"", s))
                        .unwrap_or_else(|| format!("string@{:04x}", index)),
                    IndexType::Type => self.resolver.resolve_type(index)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("type@{:04x}", index)),
                    IndexType::Method => self.resolver.resolve_method(index)
                        .unwrap_or_else(|| format!("meth@{:04x}", index)),
                    IndexType::Field => self.resolver.resolve_field(index)
                        .map(|f| format!("{}->{}:{}", f.class, f.name, f.type_name))
                        .unwrap_or_else(|| format!("field@{:04x}", index)),
                    IndexType::None => String::new(),
                };

                if description.contains("string@") { description = description.replace("string@", &format!("{}", resolved)); }
                else if description.contains("type@") { description = description.replace("type@", &format!("{}", resolved)); }
                else if description.contains("meth@") { description = description.replace("meth@", &format!("{}", resolved)); }
                else if description.contains("field@") { description = description.replace("field@", &format!("{}", resolved)); }
                else { description = format!("{} {}", description, resolved); }
            }
        }

        let instruction = Instruction {
            offset: current_instr_byte_addr,
            opcode: opcode_byte,
            name: info.name,
            description,
        };

        (instruction, info.length * 2)
    }
}
