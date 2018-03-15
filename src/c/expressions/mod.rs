pub mod binary;
pub mod unary;

use std::io;

use super::{Compile, Scope};

pub use self::binary::*;
pub use self::unary::*;

#[derive(Clone,Debug,PartialEq)]
pub enum Expression {
    Assign(String, Box<Expression>),
    Var(String),
    Constant(i32),
    UnOp(UnaryOperator, Box<Expression>),
    BinOp(BinaryOperator, Box<Expression>, Box<Expression>),
}

impl Compile for Expression {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope) -> io::Result<()> where O: io::Write {
        match *self {
            Expression::Assign(ref name, ref exp) => {
                if !scope.contains(name) {
                    return Err(io::ErrorKind::NotFound.into());
                }

                exp.compile(output, scope)?;

                let index = scope.get_variable_index(name).unwrap();
                let offset = scope.get_size() - index;
                output.write_fmt(format_args!("movl %eax, -{}(%ebp)\n", offset))
            },
            Expression::Var(ref name) => {
                if !scope.contains(name) {
                    return Err(io::ErrorKind::NotFound.into());
                }

                let index = scope.get_variable_index(name).unwrap();
                let offset = scope.get_size() - index;
                output.write_fmt(format_args!("movl -{}(%ebp), %eax\n", offset))
            },
            Expression::Constant(i) => output.write_fmt(format_args!("movl ${}, %eax\n", i)),
            Expression::UnOp(ref op, ref expr) => {
                expr.compile(output, scope)?;
                op.compile(output, scope)
            },
            Expression::BinOp(ref op, ref lval, ref rval) => {
                lval.compile(output, scope)?;
                output.write_all(b"push %eax\n")?;
                rval.compile(output, scope)?;
                output.write_all(b"pop %ecx\n")?;
                op.compile(output, scope)
            },
        }
    }
}
