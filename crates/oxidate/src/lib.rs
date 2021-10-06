//! A prototype implementation of the Rust syntax from scratch (rather than
//! newtypes of `syn`) that support derive(Parse), derive(ToTokens) and other
//! conveniences across all AST node types.

extern crate proc_macro;

pub mod ast;
pub mod coerce;
pub mod error;
pub mod lex;
pub mod tokens;
pub mod traits;

#[macro_use]
mod macros;

pub use self::ast::span::Span;
pub use self::lex::literal::{literal_types::LiteralTrait, Literal, UnspannedLiteral};
pub use self::tokens::{ParseBuffer, Tokens};
pub use self::traits::*;

pub mod tools {
    pub use {::proc_macro2, ::proc_macro_error, ::quote};
}
