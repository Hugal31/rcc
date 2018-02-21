use std::io::{Result, Write};

pub mod expressions;
pub mod function;
pub mod instructions;
pub mod types;

pub use self::expressions::Expression;
pub use self::function::*;

pub trait ToAsm {
    fn to_asm<O>(&self, output: &mut O) -> Result<()> where O: Write;
}
