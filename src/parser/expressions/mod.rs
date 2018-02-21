use std::str::FromStr;

use nom::IResult::Done;

use nom::digit;

named!(pub parse_i32<&str, i32>, map_res!(digit, i32::from_str));

#[test]
fn test_parse_i32() {
    assert_eq!(parse_i32("42"), Done("", 42));
}
