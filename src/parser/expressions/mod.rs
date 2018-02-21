use std::str::FromStr;

use nom::{IResult, digit};

pub fn parse_expression(input: &str) -> IResult<&str, i32> {
    parse_i32(input)
}

named!(pub parse_i32<&str, i32>, map_res!(digit, i32::from_str));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("42"), Done("", 42));
    }
}
