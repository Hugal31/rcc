use std::fmt;
use std::io;
use std::str::FromStr;

use c::{Compile, Scope};

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
//    Modulo,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
}

impl Compile for BinaryOperator {
    // RValue should be in ECX, LValue in EAX
    fn compile<O>(&self, output: &mut O, _scope: &mut Scope) -> io::Result<()> where O: io::Write {
        match *self {
            BinaryOperator::Addition => output.write_all(b"addl %ecx, %eax\n"),
            BinaryOperator::Subtraction => output.write_all(b"xchg %ecx, %eax
subl %ecx, %eax\n"),
            BinaryOperator::Multiplication => output.write_all(b"imul %ecx, %eax\n"),
            BinaryOperator::Division => output.write_all(b"xchg %ecx, %eax
xor %edx, %edx
divl %ecx\n"),
//    &BinaryOperator::Modulo => {},
            BinaryOperator::LessThan |
            BinaryOperator::GreaterThan |
            BinaryOperator::LessOrEqual |
            BinaryOperator::GreaterOrEqual |
            BinaryOperator::Equal |
            BinaryOperator::NotEqual => {
                output.write_all(b"cmpl %eax, %ecx\n")?;
                let opcode: &[u8] = match *self {
                    BinaryOperator::LessThan => b"setl",
                    BinaryOperator::GreaterThan => b"setg",
                    BinaryOperator::LessOrEqual => b"setle",
                    BinaryOperator::GreaterOrEqual => b"setge",
                    BinaryOperator::Equal => b"sete",
                    BinaryOperator::NotEqual => b"setne",
                    _ => unreachable!(),
                };
                output.write_all(opcode)?;
                output.write_all(b" %al\n")
            },
            BinaryOperator::LogicalAnd => output.write_all(b"cmpl $0, %ecx
setne %cl
cmpl $0, %eax
movl $0, %eax
setne %al
andb %cl, %al\n"),
            BinaryOperator::LogicalOr => output.write_all(b"orl %ecx, %eax
movl $0, %eax
setne %al\n")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBinaryOperationError {}

impl fmt::Display for ParseBinaryOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a binary operator".fmt(f)
    }
}

impl FromStr for BinaryOperator {
    type Err = ParseBinaryOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+"  => Ok(BinaryOperator::Addition),
            "-"  => Ok(BinaryOperator::Subtraction),
            "*"  => Ok(BinaryOperator::Multiplication),
            "/"  => Ok(BinaryOperator::Division),
            //"%"  => Ok(BinaryOperator::Modulo),
            "<"  => Ok(BinaryOperator::LessThan),
            ">"  => Ok(BinaryOperator::GreaterThan),
            "<=" => Ok(BinaryOperator::LessOrEqual),
            ">=" => Ok(BinaryOperator::GreaterOrEqual),
            "==" => Ok(BinaryOperator::Equal),
            "!=" => Ok(BinaryOperator::NotEqual),
            "&&" => Ok(BinaryOperator::LogicalAnd),
            "||" => Ok(BinaryOperator::LogicalOr),
            _    => Err(ParseBinaryOperationError{}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::BinaryOperator::*;

    #[test]
    fn test_parse() {
        assert_eq!("+".parse(), Ok(Addition));
        assert_eq!("-".parse(), Ok(Subtraction));
        assert_eq!("*".parse(), Ok(Multiplication));
        assert_eq!("/".parse(), Ok(Division));
        assert_eq!("<".parse(), Ok(LessThan));
        assert_eq!(">".parse(), Ok(GreaterThan));
        assert_eq!("<=".parse(), Ok(LessOrEqual));
        assert_eq!(">=".parse(), Ok(GreaterOrEqual));
        assert_eq!("&&".parse(), Ok(LogicalAnd));
        assert_eq!("||".parse(), Ok(LogicalOr));
        assert_eq!("nop".parse::<BinaryOperator>(), Err(ParseBinaryOperationError{}));
    }
}
