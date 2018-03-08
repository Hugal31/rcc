use std::io::{Result, Write};

use super::Compile;
use super::expressions::AdditiveExpression;

#[derive(Debug,PartialEq)]
pub enum Statement {
    Return(AdditiveExpression)
}

impl Compile for Statement {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        match *self {
            Statement::Return(ref e) => {
                e.compile(output)?;
                output.write_all(b"ret\n")?;
            },
        }

        Ok(())
    }
}
