pub mod expressions;
pub mod function;
pub mod statement;
pub mod types;

pub use self::expressions::*;
pub use self::function::*;
pub use self::statement::*;
pub use self::types::*;

pub const KEYWORDS: &[&str] = &[
    // Types
    "int",
    "void",
    // Other keywords
    "return",
];

pub type AST = Function;
