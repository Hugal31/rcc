use std::io::{Result, Write};

use super::Compile;
use super::expressions::Expression;

#[derive(Debug,PartialEq)]
pub enum Statement {
    Return(Expression)
}

impl Compile for Statement {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        match self {
            &Statement::Return(ref e) => {
                e.compile(output)?;
                output.write("ret\n".as_bytes())?;
            },
        }

        Ok(())
    }
}
