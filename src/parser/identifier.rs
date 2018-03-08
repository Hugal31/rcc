use nom::{ErrorKind, IResult, IResult::*, Needed};

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
                return Done(&input[idx..], &input[0..idx])
            }
        }
    }
    Done(&input[input_length..], &input[0..input_length])
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
    }
}
