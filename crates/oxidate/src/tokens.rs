use derive_new::new;
use peekmore::{PeekMore, PeekMoreIterator};
use proc_macro2::{self};

use crate::lex::{Lexed, Punctuation, Word};

#[derive(Debug, Default, new)]
pub struct Tokens {
    inner: proc_macro2::TokenStream,
}

impl Tokens {
    pub fn empty() -> Tokens {
        Default::default()
    }

    pub fn parse(self) -> ParseBuffer {
        ParseBuffer {
            inner: self.inner.into_iter().peekmore(),
        }
    }
}

impl From<proc_macro2::TokenStream> for Tokens {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        Tokens::new(stream)
    }
}

impl From<proc_macro::TokenStream> for Tokens {
    fn from(stream: proc_macro::TokenStream) -> Self {
        Tokens::new(stream.into())
    }
}

impl Into<proc_macro2::TokenStream> for Tokens {
    fn into(self) -> proc_macro2::TokenStream {
        self.inner
    }
}

impl Into<proc_macro::TokenStream> for Tokens {
    fn into(self) -> proc_macro::TokenStream {
        let inner: proc_macro2::TokenStream = self.inner.into();
        inner.into()
    }
}

#[derive(Debug)]
pub struct ParseBuffer {
    inner: PeekMoreIterator<proc_macro2::token_stream::IntoIter>,
}

impl Default for ParseBuffer {
    fn default() -> Self {
        Self::new(proc_macro2::TokenStream::default())
    }
}

#[derive(Debug, Clone)]
pub enum PeekedToken<'a> {
    EOF,
    Lexed(Lexed<'a>),
}

impl<'a> PeekedToken<'a> {
    pub fn as_punctuation(&self, char: &str) -> Option<&Punctuation> {
        match self {
            PeekedToken::Lexed(lexed) => lexed.as_punctuation(char),
            _ => None,
        }
    }

    pub fn as_word(&self, word: &str) -> Option<&Word> {
        match self {
            PeekedToken::Lexed(lexed) => lexed.as_word(word),
            _ => None,
        }
    }
}

impl ParseBuffer {
    pub fn new(stream: impl Into<proc_macro2::TokenStream>) -> ParseBuffer {
        ParseBuffer {
            inner: stream.into().into_iter().peekmore(),
        }
    }

    pub fn peek(&mut self) -> PeekedToken {
        match self.inner.peek() {
            Some(token) => {
                let token: Lexed = token.into();
                PeekedToken::Lexed(token)
            }
            None => PeekedToken::EOF,
        }
    }
}
