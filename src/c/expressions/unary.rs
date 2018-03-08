use std::fmt;
use std::io;
use std::str::FromStr;
use c::Compile;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum UnaryOperator {
    Negation,
    Bitwise,
    LocalNegation,
}

impl Compile for UnaryOperator {
    fn compile<O>(&self, output: &mut O) -> io::Result<()> where O: io::Write {
        match *self {
            UnaryOperator::Negation => {
                output.write_all(b"neg %eax\n")?;
            },
            UnaryOperator::Bitwise => {
                output.write_all(b"not %eax\n")?;
            },
            UnaryOperator::LocalNegation => {
                output.write_all(b"cmpl $0, %eax\nmovl $0, %eax\nsete %al\n")?;
            },
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseUnaryOperatorError {}

impl fmt::Display for ParseUnaryOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not '~', '-' or '!'".fmt(f)
    }
}

impl FromStr for UnaryOperator {
    type Err = ParseUnaryOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "~" => Ok(UnaryOperator::Bitwise),
            "!" => Ok(UnaryOperator::LocalNegation),
            "-" => Ok(UnaryOperator::Negation),
            _ => Err(ParseUnaryOperatorError{}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c::tests::test_compile;

    #[test]
    fn test_compile_bitwise() {
        test_compile(UnaryOperator::Bitwise, "not %eax\n");
    }

    #[test]
    fn test_compile_negation() {
        test_compile(UnaryOperator::Negation, "neg %eax\n");
    }

    #[test]
    fn test_compile_local_negation() {
        test_compile(UnaryOperator::LocalNegation, "cmpl $0, %eax\nmovl $0, %eax\nsete %al\n");
    }

    #[test]
    fn test_parse() {
        assert_eq!("~".parse(), Ok(UnaryOperator::Bitwise));
        assert_eq!("!".parse(), Ok(UnaryOperator::LocalNegation));
        assert_eq!("-".parse(), Ok(UnaryOperator::Negation));
        assert_eq!("nop".parse::<UnaryOperator>(), Err(ParseUnaryOperatorError{}));
    }
}
