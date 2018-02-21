#[macro_use] extern crate nom;

mod c;
mod parser;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, &'static str>;

pub fn compile_file(input_file: &str, output_file: &str) -> Result<()> {
    let mut input = File::open(input_file).map_err(|_| "Invalid file")?;

    compile(&mut input, &mut std::io::stdout())
}

fn compile<I: Read, O: Write>(input: &mut I, output: &mut O) -> Result<()> {
    let mut data = String::new();
    input.read_to_string(&mut data).unwrap();

    let function = parser::parse(&data).map_err(|_| "Error while parsing")?;
    output.write(b".globl ");
    output.write(function.name.as_bytes());
    output.write("\n".as_bytes());
    output.write(function.name.as_bytes());
    output.write(b":\n");
    output.write(b"\tmovl $");
    output.write(function.instructions[0].expression.to_string().as_bytes());
    output.write(b", %eax\n\tret\n");

    Ok(())
}
