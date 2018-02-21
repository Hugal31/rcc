use c::statement::Statement;

mod ret;

use self::ret::parse_return;

named!(pub parse_statement<&str, Statement>,
    ws!(do_parse!(
        inst: parse_return >>
        char!(';') >>
        (inst)
    ))
);
