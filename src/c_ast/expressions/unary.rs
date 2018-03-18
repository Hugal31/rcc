use std::result::Result as StdResult;
use std::fmt;
use std::str::FromStr;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum UnaryOperator {
    Negation,
    Bitwise,
    LocalNegation,
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

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
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

    #[test]
    fn test_parse() {
        assert_eq!("~".parse(), Ok(UnaryOperator::Bitwise));
        assert_eq!("!".parse(), Ok(UnaryOperator::LocalNegation));
        assert_eq!("-".parse(), Ok(UnaryOperator::Negation));
        assert_eq!("nop".parse::<UnaryOperator>(), Err(ParseUnaryOperatorError{}));
    }
}
