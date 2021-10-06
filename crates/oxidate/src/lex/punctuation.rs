use std::borrow::Cow;

use oxidate_macros::data;
use proc_macro2::Punct;
use quote::ToTokens;

use crate::{AstSpan, ToTokenTree};

#[data]
pub struct Punctuation<'a> {
    inner: Cow<'a, Punct>,
}

impl<'a> AstSpan for Punctuation<'a> {
    type Unspanned = Self;

    fn span(&self) -> crate::Span {
        self.inner.span().into()
    }

    fn unspanned(&self) -> &Self::Unspanned {
        self
    }
}

impl<'a> ToTokenTree for Punctuation<'a> {
    fn to_tt(&self) -> proc_macro2::TokenTree {
        proc_macro2::TokenTree::Punct(self.inner.clone().into_owned())
    }
}

impl<'a> ToTokens for Punctuation<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

impl<'a> Punctuation<'a> {
    pub fn as_char(&self) -> char {
        self.inner.as_char()
    }
}

impl PartialEq<char> for Punctuation<'_> {
    fn eq(&self, other: &char) -> bool {
        self.inner.as_char() == *other
    }
}

impl PartialEq<str> for Punctuation<'_> {
    fn eq(&self, other: &str) -> bool {
        if other.len() != 1 {
            false
        } else {
            other.chars().nth(0).unwrap() == self.inner.as_char()
        }
    }
}

impl PartialEq<char> for &Punctuation<'_> {
    fn eq(&self, other: &char) -> bool {
        self.inner.as_char() == *other
    }
}

impl PartialEq<str> for &Punctuation<'_> {
    fn eq(&self, other: &str) -> bool {
        if other.len() != 1 {
            false
        } else {
            other.chars().nth(0).unwrap() == self.inner.as_char()
        }
    }
}

impl Into<Punctuation<'static>> for Punct {
    fn into(self) -> Punctuation<'static> {
        Punctuation {
            inner: Cow::Owned(self),
        }
    }
}

impl<'a> Into<Punctuation<'a>> for &'a Punct {
    fn into(self) -> Punctuation<'a> {
        Punctuation {
            inner: Cow::Borrowed(self),
        }
    }
}
