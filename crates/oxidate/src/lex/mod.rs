pub mod literal;
pub mod punctuation;
pub mod word;

use std::borrow::Cow;

use ::proc_macro2::{self, Group};
use oxidate_macros::data;
use quote::ToTokens;

use crate::AstSpan;

pub use self::{
    literal::{Literal, UnspannedLiteral},
    punctuation::Punctuation,
    word::Word,
};

#[data]
pub enum Lexed<'a> {
    Delimited(Delimited<'a>),
    Word(Word<'a>),
    Punctuation(Punctuation<'a>),
    Literal(Literal),
}

impl<'a> AstSpan for Lexed<'a> {
    type Unspanned = Self;

    fn span(&self) -> crate::Span {
        match self {
            Lexed::Delimited(delimited) => delimited.span(),
            Lexed::Word(word) => word.span(),
            Lexed::Punctuation(punctuation) => punctuation.span(),
            Lexed::Literal(literal) => literal.span(),
        }
    }

    fn unspanned(&self) -> &Self::Unspanned {
        self
    }
}

impl<'a> Lexed<'a> {
    pub fn as_punctuation(&self, char: &str) -> Option<&Punctuation> {
        match self {
            Lexed::Punctuation(p) if p == char => Some(p),
            _ => None,
        }
    }

    pub fn as_word(&self, word: &str) -> Option<&Word> {
        match self {
            Lexed::Word(ident) if ident == word => Some(ident),
            _ => None,
        }
    }
}

impl Into<Lexed<'static>> for proc_macro2::TokenTree {
    fn into(self) -> Lexed<'static> {
        match self {
            proc_macro2::TokenTree::Group(group) => Lexed::Delimited(group.into()),
            proc_macro2::TokenTree::Ident(ident) => Lexed::Word(ident.into()),
            proc_macro2::TokenTree::Punct(punct) => Lexed::Punctuation(punct.into()),
            proc_macro2::TokenTree::Literal(literal) => Lexed::Literal(literal.into()),
        }
    }
}

impl<'a> Into<Lexed<'a>> for &'a proc_macro2::TokenTree {
    fn into(self) -> Lexed<'a> {
        match self {
            proc_macro2::TokenTree::Group(group) => Lexed::Delimited(group.into()),
            proc_macro2::TokenTree::Ident(ident) => Lexed::Word(ident.into()),
            proc_macro2::TokenTree::Punct(punct) => Lexed::Punctuation(punct.into()),
            proc_macro2::TokenTree::Literal(literal) => Lexed::Literal(literal.into()),
        }
    }
}

#[data]
pub struct Delimited<'a> {
    inner: Cow<'a, Group>,
}

impl<'a> AstSpan for Delimited<'a> {
    type Unspanned = Self;

    fn span(&self) -> crate::Span {
        self.inner.span().into()
    }

    fn unspanned(&self) -> &Self::Unspanned {
        self
    }
}

impl<'a> ToTokens for Delimited<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

impl Into<Delimited<'static>> for Group {
    fn into(self) -> Delimited<'static> {
        Delimited {
            inner: Cow::Owned(self),
        }
    }
}

impl<'a> Into<Delimited<'a>> for &'a Group {
    fn into(self) -> Delimited<'a> {
        Delimited {
            inner: Cow::Borrowed(self),
        }
    }
}
