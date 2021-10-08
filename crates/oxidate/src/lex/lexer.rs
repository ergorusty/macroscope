use std::{fmt::Display, str::FromStr};

use derive_new::new;
use oxidate_macros::{base, data};
use peekmore::{PeekMore, PeekMoreIterator};
use proc_macro2::{self, LexError, TokenStream};

use crate::{
    lex::{LexNode, Punctuation, Word},
    AstParse, AstSpan, Literal, ParseOutcome, Span, Spanned,
};

#[derive(Debug, Default, new)]
pub struct Tokens {
    inner: proc_macro2::TokenStream,
}

impl Tokens {
    pub fn empty() -> Tokens {
        Default::default()
    }

    pub fn parse(self) -> Lexer {
        let iterator = self.inner.into_iter().peekmore();
        Lexer { inner: iterator }
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

type LexIterator = PeekMoreIterator<proc_macro2::token_stream::IntoIter>;

#[derive(Debug)]
pub struct Lexer {
    inner: LexIterator,
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new(proc_macro2::TokenStream::default())
    }
}

#[derive(Debug, Clone)]
pub enum PeekedToken<'a> {
    EOF,
    Lexed(LexNode<'a>),
}

impl<'a> AstSpan for PeekedToken<'a> {
    type Unspanned = Self;

    fn span(&self) -> Span {
        match self {
            PeekedToken::EOF => Span::eof(),
            PeekedToken::Lexed(node) => node.span(),
        }
    }

    fn unspanned(&self) -> &Self::Unspanned {
        self
    }

    fn into_unspanned(self) -> Self::Unspanned {
        self
    }
}

#[data(copy)]
pub enum TransactionState {
    Started,
    Committed,
    RolledBack,
}

#[base]
pub struct PeekTransaction<'a> {
    lexer: &'a mut Lexer,
    state: TransactionState,
}

impl<'a> PeekTransaction<'a> {
    fn new(lexer: &'a mut Lexer) -> PeekTransaction<'a> {
        PeekTransaction {
            lexer,
            state: TransactionState::Started,
        }
    }

    pub fn as_punctuation(&mut self, char: &str) -> Option<Punctuation> {
        let char = char.chars().nth(0).unwrap();

        self.lexer.lookahead().as_punctuation(char).cloned()
    }

    pub fn is_punctuation(&mut self, char: &str) -> bool {
        let char = char.chars().nth(0).unwrap();

        self.lexer.lookahead().as_punctuation(char).is_some()
    }

    pub fn as_any_punctuation(&mut self) -> Option<Punctuation> {
        self.lexer.lookahead().as_any_punctuation().cloned()
    }

    pub fn as_word(&mut self, word: &str) -> Option<Word> {
        self.lexer.lookahead().as_word(word).cloned()
    }

    pub fn is_word(&mut self, word: &str) -> bool {
        self.lexer.lookahead().as_word(word).is_some()
    }

    pub fn as_any_word(&mut self) -> Option<Word> {
        self.lexer.lookahead().as_any_word().cloned()
    }

    pub fn as_literal(&mut self) -> Option<Literal> {
        self.lexer.lookahead().as_literal().cloned()
    }

    pub fn is_literal(&mut self) -> bool {
        self.lexer.lookahead().as_literal().is_some()
    }

    pub fn commit<U>(mut self, output: U) -> ParseOutcome<U>
    where
        U: AstParse + 'a,
    {
        self.state = TransactionState::Committed;
        let span = self.lexer.lookahead().span();
        self.lexer.commit();
        ParseOutcome::Success(Spanned::wrap(output, span))
    }

    pub fn rollback<U>(mut self) -> ParseOutcome<U>
    where
        U: AstParse,
    {
        self.state = TransactionState::RolledBack;
        self.lexer.rollback();
        ParseOutcome::LookaheadMismatch
    }
}

impl<'a> Drop for PeekTransaction<'a> {
    fn drop(&mut self) {
        match self.state {
            TransactionState::Started => self.lexer.rollback(),
            _ => {}
        }
    }
}

impl<'a> Display for PeekedToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeekedToken::EOF => write!(f, "EOF"),
            PeekedToken::Lexed(node) => write!(f, "{}", node),
        }
    }
}

impl<'a> PeekedToken<'a> {
    pub fn as_punctuation(&self, char: char) -> Option<&Punctuation> {
        match &self {
            PeekedToken::Lexed(lexed) => lexed.as_punctuation()?.as_specific(char),
            _ => None,
        }
    }

    pub fn as_any_punctuation(&self) -> Option<&Punctuation> {
        match self {
            PeekedToken::Lexed(lexed) => lexed.as_punctuation(),
            _ => None,
        }
    }

    pub fn as_word(&self, word: &str) -> Option<&Word> {
        let as_word = self.as_any_word()?;

        as_word.as_keyword(word)
    }

    pub fn as_any_word(&self) -> Option<&Word> {
        match self {
            PeekedToken::Lexed(lexed) => lexed.as_word(),
            _ => None,
        }
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            PeekedToken::Lexed(lexed) => lexed.as_literal(),
            _ => None,
        }
    }
}

impl Lexer {
    pub fn source(buffer: &str) -> Result<Lexer, LexError> {
        let stream = TokenStream::from_str(buffer)?;

        Ok(Lexer::new(stream))
    }

    pub fn new(stream: impl Into<proc_macro2::TokenStream>) -> Lexer {
        let iterator = stream.into().into_iter().peekmore();

        Lexer { inner: iterator }
    }

    pub fn is_eof(&mut self) -> bool {
        self.inner.peek().is_none()
    }

    pub fn try_parse<T>(&mut self) -> ParseOutcome<T>
    where
        T: AstParse,
    {
        T::try_parse(self)
    }

    pub fn lookahead_is<T>(&mut self) -> bool
    where
        T: AstParse,
    {
        T::check(self)
    }

    pub fn next_span(&mut self) -> Span {
        match self.inner.peek() {
            Some(token) => token.span().into(),
            None => Span::eof(),
        }
    }

    pub fn lookahead(&mut self) -> PeekedToken {
        match self.inner.peek() {
            Some(token) => {
                let token: LexNode = token.into();
                PeekedToken::Lexed(token)
            }
            None => PeekedToken::EOF,
        }
    }

    pub fn begin<'a>(&'a mut self) -> PeekTransaction<'a> {
        PeekTransaction::new(self)
    }

    pub fn commit(&mut self) {
        self.inner.next();
    }

    pub fn rollback(&mut self) {}
}
