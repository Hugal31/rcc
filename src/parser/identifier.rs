use nom::{ErrorKind, IResult, IResult::*, Needed};

use c::KEYWORDS;

pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    let input_length = input.len();
    if input_length == 0 {
        return Incomplete(Needed::Unknown);
    }

    for (idx, item) in input.char_indices() {
        if !(item == '_' || item.is_alphabetic()
            || idx != 0 && item.is_digit(10)) {
            if idx == 0 {
                return Error(error_position!(ErrorKind::Custom(0), input))
            } else {
                return err_if_keyword(&input[idx..], &input[0..idx]);
            }
        }
    }

    err_if_keyword(&input[input_length..], &input[0..input_length])
}

fn err_if_keyword<'a>(remaining: &'a str, var_name: &'a str) -> IResult<&'a str, &'a str> {
    if KEYWORDS.contains(&var_name) {
        Error(error_position!(ErrorKind::Custom(0), var_name))
    } else {
        Done(remaining, var_name)
    }
}

pub fn continue_ident(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        return IResult::Incomplete(Needed::Size(1));
    }

    if is_alphanumeric_or_underscore(input.chars().next().unwrap()) {
        IResult::Done(&input[1..], &input[..1])
    } else {
        IResult::Error(error_position!(ErrorKind::Custom(1), input))
    }
}

fn is_alphanumeric_or_underscore(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_parse_identifier() {
        assert_eq!(parse_identifier("abcdef1234"), Done("", "abcdef1234"));
        assert_eq!(parse_identifier("_abcdef1234"), Done("", "_abcdef1234"));
        assert_eq!(parse_identifier("_abcdef1234 "), Done(" ", "_abcdef1234"));
        assert_eq!(parse_identifier("1abc"), Error(ErrorKind::Custom(0)));
        assert_eq!(parse_identifier(""), Incomplete(Needed::Unknown));
        assert!(parse_identifier("int").is_err());
        assert!(parse_identifier("int ").is_err());
    }
}
