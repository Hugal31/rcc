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
        .arg(Arg::with_name("ASSEMBLY")
            .short("s")
            .help("Output assembly"))
        .get_matches();

    let assembly = matches.is_present("ASSEMBLY");
    let input_file = matches.value_of("INPUT").unwrap();
    let default_output = get_output_file_name(input_file, assembly);
    let output_file = matches.value_of("OUTPUT").unwrap_or(&default_output);
    if let Err(e) = rcc::compile_file(input_file, output_file, assembly) {
        eprintln!("Error: {:?}", e);
        ::std::process::exit(1);
    }
}

fn get_output_file_name(input_file_name: &str, assembly: bool) -> String {
    let outfile = if let Some(idx) = input_file_name.rfind('.') {
        &input_file_name[..idx]
    } else {
        "a.out"
    }.to_owned();

    if assembly {
        outfile + ".s"
    } else {
        outfile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_file_name() {
        assert_eq!(get_output_file_name("test.c", false), "test");
        assert_eq!(get_output_file_name("-", false), "a.out");
        assert_eq!(get_output_file_name(".src/test.c", false), ".src/test");
    }

    #[test]
    fn test_get_output_assembly_file_name() {
        assert_eq!(get_output_file_name("test.c", true), "test.s");
    }
}
