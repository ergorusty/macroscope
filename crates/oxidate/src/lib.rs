//! A prototype implementation of the Rust syntax from scratch (rather than
//! newtypes of `syn`) that support derive(Parse), derive(ToTokens) and other
//! conveniences across all AST node types.

extern crate proc_macro;

pub mod ast;
pub mod coerce;
pub mod error;
pub mod lex;
pub mod traits;

#[cfg(test)]
mod tests;

#[macro_use]
mod macros;

pub use self::ast::{
    span::{Span, Spannable, Spanned},
    token::{RustKeyword, RustOperator},
};
pub use self::lex::literal::{literal_types::LiteralTrait, Literal};
pub use self::lex::{
    lexer::{Lexer, Tokens},
    Delimited, LexNode, Punctuation, Word,
};
pub use self::traits::*;

#[allow(non_camel_case_types)]
pub type string = &'static str;

pub mod tools {
    pub use {::proc_macro2, ::proc_macro_error, ::quote};
}
