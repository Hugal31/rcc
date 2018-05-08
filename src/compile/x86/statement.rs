use std::io::Write;

use c_ast::*;
use compile::*;

const DEFAULT_VALUE: Expression = Expression::Constant(0);

impl Compile for Statement {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope) -> Result<()>
    where
        O: Write,
    {
        match *self {
            Statement::Return(ref e) => {
                e.compile(output, scope)?;
                write_epilogue(output).map_err(|e| e.into())
            }
            Statement::Declare(ref name, ref exp) => {
                if scope.contains(name) {
                    return Err(ErrorKind::VariableAlreadyExists.into()); // TODO Use custom errors !
                }
                exp.as_ref()
                    .unwrap_or(&DEFAULT_VALUE)
                    .compile(output, scope)?;
                scope.add_variable(Variable::new(name));
                output.write_all(b"pushl %eax\n").map_err(|e| e.into())
            }
            Statement::Exp(ref exp) => exp.compile(output, scope),
        }
    }
}