#[macro_use] extern crate clap;
extern crate rcc;

use clap::{Arg, App};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(crate_version!())
        .arg(Arg::with_name("INPUT")
             .help("Set the input file to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("Output file")
             .default_value("out.s")
             .index(2))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    rcc::compile_file(input_file, matches.value_of("OUTPUT").unwrap());
}
