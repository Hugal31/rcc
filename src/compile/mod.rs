mod scope;
mod x86;

pub use self::x86::*;
pub use self::{errors::*, scope::*};

pub mod errors {
    #[cfg(intellij_type_hinting)]
    pub use error_chain_for_dumb_ides::stubs::*;

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
    use super::{Compile, Scope};

    pub fn test_compile<C>(expr: C, expected_output: &str)
    where
        C: Compile,
    {
        let mut scope = Scope::new();
        test_compile_with_scope(expr, &mut scope, expected_output)
    }

    pub fn test_compile_with_scope<C>(expr: C, scope: &mut Scope, expected_output: &str)
    where
        C: Compile,
    {
        let mut buffer = Vec::<u8>::with_capacity(256);
        expr.compile(&mut buffer, scope).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), expected_output);
    }
}
