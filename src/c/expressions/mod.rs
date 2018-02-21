pub mod unary;

use std::io::{Result, Write};
use c::Compile;
pub use self::unary::UnaryOperator;

#[derive(Debug,PartialEq)]
pub enum Expression {
    Literal(i32),
    Unary(UnaryOperator, Box<Expression>),
}

impl Compile for Expression {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        match self {
            &Expression::Literal(i) => {
                output.write(format!("movl ${}, %eax\n", i).as_bytes())?;
            },
            &Expression::Unary(op, ref exp) => {
                exp.compile(output)?;
                op.compile(output)?;
            },
        }

        Ok(())
    }
}

impl From<i32> for Expression {
    fn from(number: i32) -> Self {
        Expression::Literal(number)
    }
}
