pub mod expressions;
pub mod function;
pub mod statement;
pub mod types;

pub use self::{expressions::*, function::*, statement::*, types::*};

pub const KEYWORDS: &[&str] = &[
    // Types
    "int",
    "void",
    // Other keywords
    "return",
];
