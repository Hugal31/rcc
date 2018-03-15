use std::io::{Result, Write};

use super::{Compile, Scope, Variable};
use super::expressions::Expression;
use super::function::write_epilogue;

#[derive(Debug,PartialEq)]
pub enum Statement {
    Return(Expression),
    Declare(String, Option<Expression>),
    Exp(Expression),
}

const DEFAULT_VALUE: Expression = Expression::Constant(0);

impl Compile for Statement {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope) -> Result<()> where O: Write {
        match *self {
            Statement::Return(ref e) => {
                e.compile(output, scope)?;
                write_epilogue(output)
            },
            Statement::Declare(ref name, ref exp) => {
                if scope.contains(name) {
                    return Err(::std::io::ErrorKind::AlreadyExists.into()); // TODO Use custom errors !
                }
                exp.as_ref().unwrap_or(&DEFAULT_VALUE).compile(output, scope)?;
                scope.add_variable(Variable::new(name));
                output.write_all(b"pushl %eax\n")
            },
            Statement::Exp(ref exp) => exp.compile(output, scope),
        }
    }
}
