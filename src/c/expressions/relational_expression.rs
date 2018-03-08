use std::io::{Result, Write};

use c::Compile;
use super::AdditiveExpression;
use super::binary::RelationalOperator;

#[derive(Debug,PartialEq)]
pub struct RelationalExpression {
    pub additive_expression: AdditiveExpression,
    pub operations: Vec<(RelationalOperator, AdditiveExpression)>,
}

impl RelationalExpression {
    #[allow(dead_code)]
    pub fn new(additive_expression: AdditiveExpression) -> RelationalExpression {
        RelationalExpression{
            additive_expression,
            operations: vec!(),
        }
    }
}

impl Compile for RelationalExpression {
    fn compile<O>(&self, output: &mut O) -> Result<()> where O: Write {
        self.additive_expression.compile(output)?;

        for &(ref operator, ref additive_expression) in &self.operations {
            output.write_all(b"push %eax\n")?;
            additive_expression.compile(output)?;
            output.write_all(b"movl %eax, %edx\npop %ecx\nmovl $0, %eax\ncmpl %edx, %ecx\n")?;
            let opcode: &[u8] = match *operator {
                RelationalOperator::LessThan => b"setl",
                RelationalOperator::GreaterThan => b"setg",
                RelationalOperator::LessOrEqual => b"setle",
                RelationalOperator::GreaterOrEqual => b"setge",
            };
            output.write_all(opcode)?;
            output.write_all(b" %al\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use c::tests::test_compile;
    use c::expressions::{Factor, Term};
    use super::*;

    #[test]
    fn test_compile_less_or_equal() {
        let twenty_one = AdditiveExpression::new(Term::new(Factor::Literal(21)));
        let fourth_two = AdditiveExpression::new(Term::new(Factor::Literal(42)));
        let expression = RelationalExpression{
            additive_expression: twenty_one,
            operations: vec![(RelationalOperator::LessThan, fourth_two)],
        };
        test_compile(expression, "movl $21, %eax
push %eax
movl $42, %eax
movl %eax, %edx
pop %ecx
movl $0, %eax
cmpl %edx, %ecx
setl %al
");
    }
}
