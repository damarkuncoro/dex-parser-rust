pub mod registers;
pub mod immediates;
pub mod branches;
pub mod special;
pub mod resolver;

pub use registers::{extract_registers, substitute_registers};
pub use immediates::{extract_immediates, substitute_immediates};
pub use branches::{extract_branch_target, substitute_branches};
pub use special::substitute_special;
pub use resolver::resolve_xref;
