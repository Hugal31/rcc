use std::io;
use std::io::Write;

use c_ast::{Expression, Function, Statement};
use compile::*;
use writers::IndentWriter;

const RETURN_0: Statement = Statement::Return(Expression::Constant(0));

impl EmitAsm for Function {
    fn emit_asm<O>(&self, output: &mut O, _ctx: &mut Context) -> Result<()>
    where
        O: io::Write,
    {
        output.write_fmt(format_args!("\t.globl {}\n", self.name))?;
        output.write_fmt(format_args!("{}:\n", self.name))?;

        // Discard upper ctx for now because global ctx is different
        let mut local_ctx = Context::new();
        let mut indent_writer = IndentWriter::new(output);
        indent_writer.write_all(
            b"push %ebp
movl %esp, %ebp\n",
        )?;
        for stmt in &self.statements {
            stmt.emit_asm(&mut indent_writer, &mut local_ctx)?;
        }

        match self.statements.last() {
            Some(&Statement::Return(_)) => (),
            _ => {
                if self.name == "main" {
                    RETURN_0.emit_asm(&mut indent_writer, &mut local_ctx)?;
                } else {
                    write_epilogue(&mut indent_writer)?;
                }
            }
        }

        Ok(())
    }
}
