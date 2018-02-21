use std::str::FromStr;

use nom::IResult::Done;

use c::types::Type;

named!(pub parse_type<&str, Type>, map_res!(alt!(tag!("void") | tag!("int")), Type::from_str));

#[test]
fn test_parse_type() {
    assert_eq!(parse_type("void"), Done("", Type::Void));
    assert_eq!(parse_type("int"), Done("", Type::Int));
}
