#[macro_use] extern crate nom;

use std::fs::File;
use std::io::prelude::*;

mod c;
mod parser;

use c::ToAsm;

type Result<T> = std::result::Result<T, &'static str>;

pub fn compile_file(input_file: &str, output_file: &str) -> Result<()> {
    let mut input = File::open(input_file).map_err(|_| "Invalid file")?;

    if output_file == "-" {
        compile(&mut input, &mut std::io::stdout())
    } else {
        let mut output = File::create(output_file).map_err(|_| "Invalid file")?;
        compile(&mut input, &mut output)
    }
}

fn compile<I, O>(input: &mut I, output: &mut O) -> Result<()> where I:Read, O: Write {
    let mut data = String::new();
    input.read_to_string(&mut data).unwrap();

    let function = parser::parse(&data).map_err(|_| "Error while parsing")?;
    function.to_asm(output).unwrap();

    Ok(())
}
