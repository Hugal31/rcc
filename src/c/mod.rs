use std::io::{Result, Write};

pub mod expressions;
pub mod function;
pub mod statement;
pub mod types;

pub use self::expressions::{Expression, Factor, Term};
pub use self::function::*;
pub use self::statement::Statement;

pub trait Compile {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write;
}

#[cfg(test)]
mod test {
    use super::Compile;

    pub fn test_compile<C>(expr: C, expected_output: &str) where C: Compile {
        let mut buffer = Vec::<u8>::new();
        expr.compile(&mut buffer).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
    }
}
