use std::io::{Result, Write};

use c::ToAsm;
use c::expressions::Expression;

#[derive(Debug, PartialEq)]
pub struct Return {
    pub expression: Expression,
}

impl ToAsm for Return {
    fn to_asm<O>(&self, output: &mut O) -> Result<()> where O: Write {
        output.write(format!("movl ${}, %eax\n", self.expression).as_bytes())?;
        output.write("ret\n".as_bytes())?;

        Ok(())
    }
}
