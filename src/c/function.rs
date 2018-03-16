use std::io;
use std::io::Write;

use c::{Compile, Expression, Scope, Statement};
use errors::*;
use writers::IndentWriter;

const RETURN_0: Statement = Statement::Return(Expression::Constant(0));

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
}

impl Compile for Function {
    fn compile<O>(&self, output: &mut O, _scope: &mut Scope) -> Result<()> where O: Write {
        output.write_fmt(format_args!("\t.globl {}\n", self.name))?;
        output.write_fmt(format_args!("{}:\n", self.name))?;

        // Discard upper scope for now because global scope is different
        let mut local_scope = Scope::new();
        let mut indent_writer = IndentWriter::new(output);
        indent_writer.write_all(b"push %ebp
movl %esp, %ebp\n")?;
        for stmt in &self.statements {
            stmt.compile(&mut indent_writer, &mut local_scope)?;
        }

        match self.statements.last() {
            Some(&Statement::Return(_)) => (),
            _ => {
                if self.name == "main" {
                    RETURN_0.compile(&mut indent_writer, &mut local_scope)?;
                } else {
                    write_epilogue(&mut indent_writer)?;
                }
            },
        }

        Ok(())
    }
}

pub fn write_epilogue<O>(output: &mut O) -> io::Result<()> where O: Write {
    output.write_all(b"movl %ebp, %esp
pop %ebp
ret\n")
}
