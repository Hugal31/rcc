use std::io::Write;

use c_ast::*;
use compile::*;

const DEFAULT_VALUE: Expression = Expression::Constant(0);

impl EmitAsm for Statement {
    fn emit_asm<O>(&self, output: &mut O, ctx: &mut Context) -> Result<()>
    where
        O: Write,
    {
        match *self {
            Statement::If {
                ref condition,
                ref then,
                ref els,
            } => {
                let condition_idx = ctx.new_condition();
                let result = condition.emit_asm(output, ctx)
                    .and_then(|()| output.write_all(b"cmpl $0, %eax\n").map_err(|e| e.into()))
                    .and_then(|()| {
                        if els.is_some() {
                            write!(output, "je _cond_{}_else\n", condition_idx)
                        } else {
                            write!(output, "je _cond_{}_end\n", condition_idx)
                        }.map_err(|e| e.into())
                    })
                    .and_then(|()| then.emit_asm(output, ctx))
                    .and_then(|()| write!(output, "jmp _cond_{}_end\n", condition_idx).map_err(|e| e.into()));
                if let Some(els) = els {
                    result.and_then(|()| write!(output, "_cond_{}_else:\n", condition_idx).map_err(|e| e.into()))
                        .and_then(|()| els.emit_asm(output, ctx))
                } else {
                    result
                }.and_then(|()| write!(output, "_cond_{}_end:\n", condition_idx).map_err(|e| e.into()))
            },
            Statement::Return(ref e) => {
                e.emit_asm(output, ctx)?;
                write_epilogue(output).map_err(|e| e.into())
            }
            Statement::Declare(ref name, ref exp) => {
                if ctx.variable_is_defined(name) {
                    return Err(ErrorKind::VariableAlreadyExists.into());
                }
                exp.as_ref()
                    .unwrap_or(&DEFAULT_VALUE)
                    .emit_asm(output, ctx)?;
                ctx.add_variable(name);
                output.write_all(b"pushl %eax\n").map_err(|e| e.into())
            }
            Statement::Exp(ref exp) => exp.emit_asm(output, ctx),
        }
    }
}
