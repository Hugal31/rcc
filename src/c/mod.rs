use std::io::{Result, Write};

pub mod expressions;
pub mod function;
pub mod statement;
pub mod types;

pub use self::expressions::Expression;
pub use self::function::*;
pub use self::statement::Statement;

pub trait Compile {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write;
}
