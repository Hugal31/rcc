#[macro_use] extern crate nom;

use std::fs::File;
use std::io::{Read, Write};

pub mod c;
pub mod parser;
mod writers;

use c::Compile;

pub fn compile_file(input_file: &str, output_file: &str) -> Result<(), &'static str> {
    let mut input = File::open(input_file).map_err(|_| "Invalid file")?;

    if output_file == "-" {
        compile(&mut input, &mut std::io::stdout())
    } else {
        let mut output = File::create(output_file).map_err(|_| "Invalid file")?;
        compile(&mut input, &mut output)
    }.map_err(|_| "Compilation_error")
}

fn compile<I, O>(input: &mut I, output: &mut O) -> Result<(), ()> where I:Read, O: Write {
    let mut data = String::new();
    input.read_to_string(&mut data).unwrap();

    let function = match parser::parse(&data) {
        Ok(func) => func,
        Err(e)   => {
            eprintln!("{}", e);
            return Err(());
        },
    };
    function.compile(output).unwrap();

    Ok(())
}
