use std::io::Write;

use c_ast::*;
use compile::*;

const DEFAULT_VALUE: Expression = Expression::Constant(0);

impl Compile for Statement {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope, compiler: &mut Compiler) -> Result<()>
    where
        O: Write,
    {
        match *self {
            Statement::If {
                ref condition,
                ref then,
                ref els,
            } => {
                let condition_idx = compiler.new_condition();
                let result = condition.compile(output, scope, compiler)
                    .and_then(|()| output.write_all(b"cmpl $0, %eax\n").map_err(|e| e.into()))
                    .and_then(|()| {
                        if els.is_some() {
                            write!(output, "je _cond_{}_else\n", condition_idx)
                        } else {
                            write!(output, "je _cond_{}_end\n", condition_idx)
                        }.map_err(|e| e.into())
                    })
                    .and_then(|()| then.compile(output, scope, compiler))
                    .and_then(|()| write!(output, "jmp _cond_{}_end\n", condition_idx).map_err(|e| e.into()));
                let result = if let Some(els) = els {
                    result.and_then(|()| write!(output, "_cond_{}_else:\n", condition_idx).map_err(|e| e.into()))
                        .and_then(|()| els.compile(output, scope, compiler))
                } else {
                    result
                };
                result.and_then(|()| write!(output, "_cond_{}_end:\n", condition_idx).map_err(|e| e.into()))
            },
            Statement::Return(ref e) => {
                e.compile(output, scope, compiler)?;
                write_epilogue(output).map_err(|e| e.into())
            }
            Statement::Declare(ref name, ref exp) => {
                if scope.contains(name) {
                    return Err(ErrorKind::VariableAlreadyExists.into());
                }
                exp.as_ref()
                    .unwrap_or(&DEFAULT_VALUE)
                    .compile(output, scope, compiler)?;
                scope.add_variable(Variable::new(name));
                output.write_all(b"pushl %eax\n").map_err(|e| e.into())
            }
            Statement::Exp(ref exp) => exp.compile(output, scope, compiler),
        }
    }
}
