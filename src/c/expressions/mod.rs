pub mod binary;
pub mod unary;

use std::io;

use super::Compile;

pub use self::binary::*;
pub use self::unary::*;

#[derive(Clone,Debug,PartialEq)]
pub enum Expression {
    Constant(i32),
    UnOp(UnaryOperator, Box<Expression>),
    BinOp(BinaryOperator, Box<Expression>, Box<Expression>),
}

impl Compile for Expression {
    fn compile<O>(&self, output: &mut O) -> io::Result<()> where O: io::Write {
        match *self {
            Expression::Constant(i) => output.write_fmt(format_args!("movl ${}, %eax\n", i)),
            Expression::UnOp(ref op, ref expr) => {
                expr.compile(output)?;
                op.compile(output)
            },
            Expression::BinOp(ref op, ref lval, ref rval) => {
                lval.compile(output)?;
                output.write_all(b"push %eax\n")?;
                rval.compile(output)?;
                output.write_all(b"pop %ecx\n")?;
                op.compile(output)
            },
        }
    }
}
