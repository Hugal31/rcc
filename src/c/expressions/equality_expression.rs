use std::fmt;
use std::io;
use std::io::Write;
use std::str::FromStr;

use c::Compile;

use super::RelationalExpression;

#[derive(Debug,PartialEq)]
pub struct EqualityExpression {
    pub relational_expression: RelationalExpression,
    pub operations: Vec<(EqualityOperator, RelationalExpression)>,
}

impl Compile for EqualityExpression {
    fn compile<O>(&self, output: &mut O) -> io::Result<()> where O: Write {
        self.relational_expression.compile(output)?;

        for &(ref operator, ref relational_expression) in &self.operations {
            output.write_all(b"push %eax\n")?;
            relational_expression.compile(output)?;
            output.write_all(b"movl %eax, %edx\npop %ecx\nmovl $0, %eax\ncmpl %edx, %ecx\n")?;
            let opcode: &[u8] = match *operator {
                EqualityOperator::Equal    => b"sete",
                EqualityOperator::NotEqual => b"setn",
            };
            output.write_all(opcode)?;
            output.write_all(b" %al\n")?;
        }

        Ok(())
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEqualityOperatorError {}

impl fmt::Display for ParseEqualityOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not '==' or '!='".fmt(f)
    }
}

impl FromStr for EqualityOperator {
    type Err = ParseEqualityOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(EqualityOperator::Equal),
            "!=" => Ok(EqualityOperator::NotEqual),
            _    => Err(ParseEqualityOperatorError{}),
        }
    }
}

#[cfg(test)]
mod tests {
    use c::expressions::Factor;
    use c::tests::test_compile;
    use super::*;

    impl From<Factor> for EqualityExpression {
        fn from(factor: Factor) -> EqualityExpression {
            EqualityExpression{
                relational_expression: RelationalExpression::from(factor),
                operations: Vec::new(),
            }
        }
    }

    #[test]
    fn test_parse_equality_operator() {
        assert_eq!("==".parse(), Ok(EqualityOperator::Equal));
        assert_eq!("!=".parse(), Ok(EqualityOperator::NotEqual));
        assert!("===".parse::<EqualityOperator>().is_err());
    }

    #[test]
    fn test_compile_equality_operator() {
        let twenty_one = RelationalExpression::from(Factor::Literal(21));
        let fourth_two = RelationalExpression::from(Factor::Literal(42));
        let expression = EqualityExpression{
            relational_expression: twenty_one,
            operations: vec![(EqualityOperator::NotEqual, fourth_two)],
        };
        test_compile(expression, "movl $21, %eax
push %eax
movl $42, %eax
movl %eax, %edx
pop %ecx
movl $0, %eax
cmpl %edx, %ecx
setn %al
");
    }
}
