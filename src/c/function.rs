use std::io::{Result, Write};

use c::Compile;
use c::Statement;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
}

impl Compile for Function {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        output.write(format!(".globl {}\n", self.name).as_bytes())?;
        output.write(format!("{}:\n", self.name).as_bytes())?;

        for stmt in &self.statements {
            stmt.compile(output)?;
        }

        Ok(())
    }
}
