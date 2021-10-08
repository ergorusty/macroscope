use std::fmt::Display;

use oxidate_macros::data;
use proc_macro2::Punct;
use quote::ToTokens;

use crate::{AstParse, AstSpan, ParseOutcome, ToTokenTree};

#[data]
pub struct Punctuation {
    inner: Punct,
}

impl Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl AstParse for Punctuation {
    fn try_parse(lexer: &mut crate::Lexer) -> ParseOutcome<Self> {
        let mut lookahead = lexer.begin();

        match lookahead.as_any_punctuation() {
            Some(punctuation) => lookahead.commit(punctuation.clone()),
            _ => lookahead.rollback(),
        }
    }

    fn check(_input: &mut crate::Lexer) -> bool {
        todo!()
    }
}

impl AstSpan for Punctuation {
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

impl ToTokenTree for Punctuation {
    fn to_tt(&self) -> proc_macro2::TokenTree {
        proc_macro2::TokenTree::Punct(self.inner.clone())
    }
}

impl ToTokens for Punctuation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_tokens(tokens)
    }
}

impl Punctuation {
    pub fn as_char(&self) -> char {
        self.inner.as_char()
    }

    pub fn as_specific(&self, punctuation: char) -> Option<&Punctuation> {
        if self.inner.as_char() == punctuation {
            Some(self)
        } else {
            None
        }
    }
}

impl PartialEq<char> for Punctuation {
    fn eq(&self, other: &char) -> bool {
        self.inner.as_char() == *other
    }
}

impl PartialEq<str> for Punctuation {
    fn eq(&self, other: &str) -> bool {
        if other.len() != 1 {
            false
        } else {
            other.chars().nth(0).unwrap() == self.inner.as_char()
        }
    }
}

impl PartialEq<char> for &Punctuation {
    fn eq(&self, other: &char) -> bool {
        self.inner.as_char() == *other
    }
}

impl PartialEq<str> for &Punctuation {
    fn eq(&self, other: &str) -> bool {
        if other.len() != 1 {
            false
        } else {
            other.chars().nth(0).unwrap() == self.inner.as_char()
        }
    }
}

impl Into<Punctuation> for Punct {
    fn into(self) -> Punctuation {
        Punctuation { inner: self }
    }
}

impl<'a> Into<Punctuation> for &'a Punct {
    fn into(self) -> Punctuation {
        Punctuation {
            inner: self.clone(),
        }
    }
}
