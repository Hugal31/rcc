mod context;
mod scope;

pub mod x86;

pub use self::x86::*;
pub use self::context::*;
pub use self::errors::*;

pub mod errors {
    error_chain! {
        errors {
            VariableAlreadyExists
            UnknownVariable
        }

        foreign_links {
            Io(::std::io::Error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EmitAsm, Context};

    pub fn test_compile<C>(expr: C, expected_output: &str)
    where
        C: EmitAsm,
    {
        let mut ctx = Context::new();
        test_compile_with_context(expr, &mut ctx, expected_output)
    }

    pub fn test_compile_with_context<C>(expr: C, ctx: &mut Context, expected_output: &str)
    where
        C: EmitAsm,
    {
        let mut buffer = Vec::<u8>::with_capacity(256);
        expr.emit_asm(&mut buffer, ctx).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
    }
}
