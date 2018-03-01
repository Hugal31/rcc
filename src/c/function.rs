use std::io::{Result, Write};

use writers::IndentWriter;
use c::Compile;
use c::Statement;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
}

impl Compile for Function {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        output.write_fmt(format_args!("\t.globl {}\n", self.name))?;
        output.write_fmt(format_args!("{}:\n", self.name))?;

        let mut indent_writer = IndentWriter::new(output);
        for stmt in &self.statements {
            stmt.compile(&mut indent_writer)?;
        }

        Ok(())
    }
}
