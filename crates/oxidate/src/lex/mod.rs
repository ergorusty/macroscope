pub mod assertions;
pub mod lexer;
pub mod literal;
pub mod punctuation;
pub mod word;

use std::{borrow::Cow, fmt::Display};

use ::proc_macro2::{self, Group};
use oxidate_macros::data;
use quote::ToTokens;

use crate::{AstSpan, Spanned};

pub use self::{literal::Literal, punctuation::Punctuation, word::Word};

#[data]
pub enum LexNode<'a> {
    Delimited(Delimited<'a>),
    Word(Word),
    Punctuation(Punctuation),
    Literal(Spanned<Literal>),
}

impl<'a> AstSpan for LexNode<'a> {
    type Unspanned = Self;

    fn span(&self) -> crate::Span {
        match self {
            LexNode::Delimited(delimited) => delimited.span().into(),
            LexNode::Word(word) => word.span().into(),
            LexNode::Punctuation(punctuation) => punctuation.span().into(),
            LexNode::Literal(literal) => literal.span().into(),
        }
    }

    fn unspanned(&self) -> &Self {
        self
    }

    fn into_unspanned(self) -> Self {
        self
    }
}

impl<'a> Display for LexNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexNode::Delimited(delimited) => write!(f, "{}", delimited),
            LexNode::Word(word) => write!(f, "{}", word),
            LexNode::Punctuation(punctuation) => write!(f, "{}", punctuation),
            LexNode::Literal(lit) => write!(f, "{}", lit.unspanned()),
        }
    }
}

impl PartialEq<Literal> for LexNode<'_> {
    fn eq(&self, other: &Literal) -> bool {
        match self {
            Self::Literal(literal) => {
                let this = literal.unspanned();
                this == other
            }
            _ => false,
        }
    }
}

impl<'a> LexNode<'a> {
    pub fn as_punctuation(&'a self) -> Option<&'a Punctuation> {
        match self {
            LexNode::Punctuation(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_word(&'a self) -> Option<&'a Word> {
        match self {
            LexNode::Word(ident) => Some(ident),
            _ => None,
        }
    }

    pub fn as_literal(&'a self) -> Option<&'a Literal> {
        match self {
            LexNode::Literal(literal) => Some(literal.unspanned()),
            _ => None,
        }
    }
}

impl Into<LexNode<'static>> for proc_macro2::TokenTree {
    fn into(self) -> LexNode<'static> {
        match self {
            proc_macro2::TokenTree::Group(group) => LexNode::Delimited(group.into()),
            proc_macro2::TokenTree::Ident(ident) => LexNode::Word(ident.into()),
            proc_macro2::TokenTree::Punct(punct) => LexNode::Punctuation(punct.into()),
            proc_macro2::TokenTree::Literal(literal) => LexNode::Literal(literal.into()),
        }
    }
}

impl<'a> Into<LexNode<'a>> for &'a proc_macro2::TokenTree {
    fn into(self) -> LexNode<'a> {
        match self {
            proc_macro2::TokenTree::Group(group) => LexNode::Delimited(group.into()),
            proc_macro2::TokenTree::Ident(ident) => LexNode::Word(ident.into()),
            proc_macro2::TokenTree::Punct(punct) => LexNode::Punctuation(punct.into()),
            proc_macro2::TokenTree::Literal(literal) => LexNode::Literal(literal.into()),
        }
    }
}

#[data]
pub struct Delimited<'a> {
    inner: Cow<'a, Group>,
}

impl<'a> Display for Delimited<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'a> AstSpan for Delimited<'a> {
    type Unspanned = Self;

    fn span(&self) -> crate::Span {
        self.inner.span().into()
    }

    fn unspanned(&self) -> &Self::Unspanned {
        self
    }

    fn into_unspanned(self) -> Self::Unspanned {
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
