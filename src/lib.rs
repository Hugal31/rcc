extern crate memchr;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

mod compile;
mod writers;

pub mod c_ast;
pub mod parser;

use std::{fs::File,
          io::Read,
          process::{Child, Command, Stdio}};

use compile::x86;

pub use errors::*;

pub mod errors {
    error_chain! {
        errors {
            SyntaxError(desc: String) {
                description("syntax error")
                display("syntax error: {}", desc)
            }
        }
        links {
            Compile(::compile::Error, ::compile::ErrorKind);
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

pub fn compile_file(input_file: &str, output_file: &str, output_assembly: bool) -> Result<()> {
    let mut input = File::open(input_file).map_err(|_| "Invalid file")?;

    let ast = get_ast(&mut input)?;

    if output_assembly {
        let mut output = File::create(output_file).map_err(|_| "Failed to create ouput file")?;

        x86::emit_asm(&ast, &mut output).map_err(|e| e.into())
    } else {
        let mut child = get_cc_command(output_file);
        x86::emit_asm(
            &ast,
            child.stdin.as_mut().expect("Failed to open stdin"),
        )?;
        child.wait().map(|_| ()).map_err(|e| e.into())
    }
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

fn get_ast<I>(input: &mut I) -> Result<c_ast::Function>
where
    I: Read,
{
    let mut data = String::new();
    input.read_to_string(&mut data).unwrap();

    parser::parse(&data).map_err(|e| ErrorKind::SyntaxError(format!("{}", e)).into())
}
