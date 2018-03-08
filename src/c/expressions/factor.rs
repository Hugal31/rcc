use std::io::{Result, Write};

use c::Compile;
use super::additive_expression::AdditiveExpression;
use super::unary::UnaryOperator;

#[derive(Debug,PartialEq)]
pub enum Factor {
    Literal(i32),
    Unary(UnaryOperator, Box<Factor>),
    Expr(Box<AdditiveExpression>),
}

impl Compile for Factor {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        match *self {
            Factor::Literal(i) => {
                output.write_all(format!("movl ${}, %eax\n", i).as_bytes())?;
            },
            Factor::Unary(op, ref exp) => {
                exp.compile(output)?;
                op.compile(output)?;
            },
            Factor::Expr(ref expr) => {
                expr.compile(output)?;
            },
        }

        Ok(())
    }
}

impl From<i32> for Factor {
    fn from(number: i32) -> Self {
        Factor::Literal(number)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use c::test::test_compile;

    #[test]
    fn test_compile_literal() {
        test_compile(Factor::Literal(42), "movl $42, %eax\n");
        test_compile(Factor::Literal(0), "movl $0, %eax\n");
        test_compile(Factor::Literal(-24), "movl $-24, %eax\n");
    }
}
