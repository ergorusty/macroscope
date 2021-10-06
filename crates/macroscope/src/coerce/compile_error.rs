use crate::impl_prelude::*;

pub trait CompileError: std::error::Error {
    fn into_compile_error(self) -> Tokens;
}

pub trait ToCompileError: CompileError {
    fn to_compile_error(&self) -> Tokens;
}

impl<T> ToCompileError for T
where
    T: CompileError + Clone,
{
    fn to_compile_error(&self) -> Tokens {
        self.clone().into_compile_error()
    }
}
