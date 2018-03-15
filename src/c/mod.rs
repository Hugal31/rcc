use std::io::{Result, Write};

mod scope;

pub mod expressions;
pub mod function;
pub mod statement;
pub mod types;

pub use self::expressions::Expression;
pub use self::function::*;
pub use self::scope::*;
pub use self::statement::Statement;

pub const KEYWORDS: &[&str] = &[
    // Types
    "int",
    "void",
    // Other keywords
    "return",
];

pub trait Compile {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope) -> Result<()> where O: Write;
}

#[cfg(test)]
mod tests {
    use super::{Compile, Scope};

    pub fn test_compile<C>(expr: C, expected_output: &str) where C: Compile {
        let mut scope = Scope::new();
        let mut buffer = Vec::<u8>::new();
        expr.compile(&mut buffer, &mut scope).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
    }
}
