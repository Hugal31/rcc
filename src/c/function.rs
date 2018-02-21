use std::io::{Result, Write};

use c::ToAsm;
use c::instructions::Return;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Return>,
}

impl ToAsm for Function {
    fn to_asm<O>(&self, output: &mut O) -> Result<()> where O: Write {
        output.write(format!(".globl {}\n", self.name).as_bytes())?;
        output.write(format!("{}:\n", self.name).as_bytes())?;

        for stmt in &self.statements {
            stmt.to_asm(output)?;
        }

        Ok(())
    }
}
