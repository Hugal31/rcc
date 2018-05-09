mod expressions;
mod function;
mod statement;

use std::io;

use c_ast::Function;
use compile::{errors::*, Context};

pub trait EmitAsm {
    fn emit_asm<O>(&self, output: &mut O, ctx: &mut Context) -> Result<()>
    where
        O: io::Write;
}

pub fn emit_asm<O>(func: &Function, mut output: O) -> Result<()>
    where
    O: io::Write,
{
    let mut context = Context::new();
    func.emit_asm(&mut output, &mut context)
}

pub fn write_epilogue<O>(output: &mut O) -> io::Result<()>
where
    O: io::Write,
{
    output.write_all(
        b"movl %ebp, %esp
pop %ebp
ret\n",
    )
}
