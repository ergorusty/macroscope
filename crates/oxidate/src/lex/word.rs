use std::{borrow::Cow, ops::Deref};

use oxidate_macros::data;
use proc_macro2::Ident;
use quote::ToTokens;

use crate::AstSpan;

#[data]
pub struct Word<'a> {
    inner: Cow<'a, Ident>,
}

impl<'a> AstSpan for Word<'a> {
    type Unspanned = Self;

    fn span(&self) -> crate::Span {
        self.inner.span().into()
    }

    fn unspanned(&self) -> &Self::Unspanned {
        self
    }
}

impl<'a> ToTokens for Word<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

impl<'a> PartialEq<str> for Word<'a> {
    fn eq(&self, other: &str) -> bool {
        self.inner.as_ref() == other
    }
}

impl<'a> Deref for Word<'a> {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Into<Word<'static>> for Ident {
    fn into(self) -> Word<'static> {
        Word {
            inner: Cow::Owned(self),
        }
    }
}

impl<'a> Into<Word<'a>> for &'a Ident {
    fn into(self) -> Word<'a> {
        Word {
            inner: Cow::Borrowed(self),
        }
    }
}
