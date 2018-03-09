extern crate memchr;

#[macro_use] extern crate nom;

use std::fs::File;
use std::io::Read;
use std::process::{Child, Command, Stdio};

pub mod c;
pub mod parser;
mod writers;

use c::Compile;

pub fn compile_file(input_file: &str, output_file: &str) -> Result<(), &'static str> {
    let mut input = File::open(input_file).map_err(|_| "Invalid file")?;

    let ast = get_ast(&mut input)
        .map_err(|_| "Compilation error")?;
    let mut child = get_cc_command(output_file);
    ast.compile(child.stdin.as_mut().expect("Failed to open stdin"))
        .map_err(|_| "Write error")
}

fn get_cc_command(output_file: &str) -> Child {
    Command::new("cc")
        .stdin(Stdio::piped())
        .args(&["-x", "assembler"])
        .arg("-m32")
        .args(&["-o", output_file])
        .arg("-")
        .spawn()
        .expect("Failed to execute command")
}

fn get_ast<I>(input: &mut I) -> Result<c::Function, ()> where I: Read {
    let mut data = String::new();
    input.read_to_string(&mut data).unwrap();

    parser::parse(&data).map_err(|e| {
        eprintln!("{}", e);
        ()
    })
}
