use std::{fmt::Display, ops::Deref};

use oxidate_macros::data;
use proc_macro2::Ident;
use quote::ToTokens;

use crate::AstSpan;

#[data]
pub struct Word {
    inner: Ident,
}

impl Word {
    pub fn as_keyword(&self, keyword: impl AsRef<str>) -> Option<&Word> {
        let inner = &self.inner;

        if inner.to_string() == keyword.as_ref() {
            Some(self)
        } else {
            None
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl AstSpan for Word {
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

impl ToTokens for Word {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

impl PartialEq<str> for Word {
    fn eq(&self, other: &str) -> bool {
        self.inner.to_string() == other
    }
}

impl Deref for Word {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Into<Word> for Ident {
    fn into(self) -> Word {
        Word { inner: self }
    }
}

impl<'a> Into<Word> for &'a Ident {
    fn into(self) -> Word {
        Word {
            inner: self.clone(),
        }
    }
}
