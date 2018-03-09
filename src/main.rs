#[macro_use] extern crate clap;
extern crate memchr;
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
             .long("output")
             .short("o")
             .help("Output file")
             .takes_value(true))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let default_output = get_output_file_name(input_file);
    let output_file = matches.value_of("OUTPUT").unwrap_or(&default_output);
    if rcc::compile_file(input_file, output_file).is_err() {
        ::std::process::exit(1);
    }
}

fn get_output_file_name(input_file_name: &str) -> String {
    if let Some(idx) = input_file_name.rfind('.') {
        &input_file_name[..idx]
    } else {
        "a.out"
    }.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_file_name() {
        assert_eq!(get_output_file_name("test.c"), "test");
        assert_eq!(get_output_file_name("-"), "a.out");
        assert_eq!(get_output_file_name(".src/test.c"), ".src/test");
    }
}
