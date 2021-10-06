use std::str::FromStr;

use derive_new::new;
use litrs::BoolLit;
use proc_macro2::{Ident, TokenTree};

use crate::Span;

#[derive(new)]
pub(crate) struct LitrsToTt<T = String>
where
    T: litrs::Buffer,
{
    literal: litrs::Literal<T>,
    span: Span,
}

impl<T> LitrsToTt<T>
where
    T: litrs::Buffer,
{
    pub(crate) fn into_tt(self) -> TokenTree {
        let Self { literal, span } = self;

        match literal {
            litrs::Literal::Bool(BoolLit::True) => {
                return TokenTree::Ident(Ident::new("true", span.into()))
            }
            litrs::Literal::Bool(BoolLit::False) => {
                return TokenTree::Ident(Ident::new("false", span.into()))
            }
            other => {
                let string = other.to_string();
                let mut upstream = proc_macro2::Literal::from_str(&string).expect(
                    "UNEXPECTED: litrs::Literals should always re-parse as proc_macro2::Literals",
                );
                upstream.set_span(self.span.into());
                TokenTree::Literal(upstream)
            }
        }
    }
}
