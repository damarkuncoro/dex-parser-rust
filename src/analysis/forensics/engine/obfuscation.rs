use crate::analysis::core::models::ScanResult;
use crate::analysis::core::visitor::{InstructionVisitor, VisitorContext};

pub struct ObfuscationVisitor {
    pub results: Vec<ScanResult>,
}

impl ObfuscationVisitor {
    pub fn new() -> Self {
        Self { results: Vec::new() }
    }

    fn check_name(&mut self, name: &str, context: &str) {
        // Detect suspiciously long names (often a sign of automated obfuscation)
        if name.len() > 128 {
            self.results.push(ScanResult {
                category: "Obfuscation: Long Name".to_string(),
                content: format!("{}: {} (Len: {})", context, name, name.len()),
                details: None,
            });
        }

        // Detect non-ASCII names (except for standard indicators)
        if name.chars().any(|c| !c.is_ascii() && c != '$' && c != '/') {
            self.results.push(ScanResult {
                category: "Obfuscation: Non-ASCII Identifier".to_string(),
                content: format!("{}: {}", context, name),
                details: None,
            });
        }
    }
}

impl InstructionVisitor for ObfuscationVisitor {
    fn visit_class(&mut self, class: &crate::dex::core::models::Class) {
        self.check_name(&class.name, "Class");
    }

    fn visit_method(&mut self, _class: &crate::dex::core::models::Class, method: &crate::dex::core::models::EncodedMethod) {
        self.check_name(&method.name, "Method");
    }

    fn visit_instruction(&mut self, ctx: &VisitorContext) {
        // Check for suspicious identifiers in instructions
        if let Some(r) = &ctx.reference {
            match r {
                crate::analysis::core::utils::Reference::Method(m) => self.check_name(m, "Method Reference"),
                crate::analysis::core::utils::Reference::Field(f) => self.check_name(f, "Field Reference"),
                crate::analysis::core::utils::Reference::Type(t) => self.check_name(t, "Type Reference"),
                _ => {}
            }
        }
    }

    fn merge(&mut self, other: Box<dyn InstructionVisitor>) {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.results.extend(other.results.clone());
        }
    }

    fn clone_factory(&self) -> Box<dyn InstructionVisitor> {
        Box::new(Self::new())
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
