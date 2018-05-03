use std::str::FromStr;

use super::identifier::continue_ident;
use c_ast::Type;

named!(pub parse_type<&str, Type>,
    do_parse!(
        t: map_res!(alt!(tag!("void") | tag!("int")), Type::from_str) >>
        not!(continue_ident) >>
        (t)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type("void"), Done("", Type::Void));
        assert_eq!(parse_type("int"), Done("", Type::Int));
        assert!(parse_type("voida").is_err());
    }
}
