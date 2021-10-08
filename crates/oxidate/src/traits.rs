use std::fmt::{Debug, Display};

use oxidate_macros::base;

use crate::{Span, Spanned};

use crate::error::MacroError;
use crate::tools::quote::ToTokens;
use crate::Lexer;

pub trait AstSpan {
    type Unspanned;

    fn span(&self) -> Span;
    fn unspanned(&self) -> &Self::Unspanned;

    fn into_unspanned(self) -> Self::Unspanned;
}

pub trait AstNode: Debug + Display + Clone + AstSpan + ToTokens {}

#[base]
pub enum ParseOutcome<T>
where
    T: AstParse,
{
    /// LookaheadMismatch means that the lookahead step didn't match. The
    /// lookahead step is infallible: it never advances the Lexer, and can
    /// reliably determine whether the lookahead matches or not
    /// non-destructively.
    LookaheadMismatch,
    /// Failure means that the lookahead succeeded, but the parser failed once
    /// the fallible part of the parsing step began.
    Failure(MacroError),
    /// The parsing step succeeded.
    Success(Spanned<T>),
    /// The parsing step succeeded, but the span is missing
    Unspanned(T),
}

impl<T> ParseOutcome<T>
where
    T: AstParse,
{
    pub fn ok(self) -> Option<T> {
        match self {
            ParseOutcome::Unspanned(value) => Some(value),
            ParseOutcome::Success(value) => Some(value.into_unspanned()),
            _ => None,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            ParseOutcome::Success(_) => true,
            _ => false,
        }
    }

    pub fn with_span(self, span: Span) -> Self {
        match self {
            ParseOutcome::Unspanned(unspanned) => {
                ParseOutcome::Success(Spanned::wrap(unspanned, span))
            }
            other => other,
        }
    }
}

impl<T> Into<Option<T>> for ParseOutcome<T>
where
    T: AstParse,
{
    fn into(self) -> Option<T> {
        self.ok()
    }
}

/// AstParse attempts to consume some tokens from the Lexer and convert them
/// into some instance of the type that implements AstParse.
pub trait AstParse: Sized {
    /// Attempt to parse. If try_parse succeeds, the Lexer will consume the
    /// tokens produced by ParseOutcome.
    fn try_parse(input: &mut Lexer) -> ParseOutcome<Self>;

    /// Check whether the next lexer token matches this type. This method is
    /// non-destructive.
    fn check(input: &mut Lexer) -> bool;
}

pub(crate) trait ToOxidateTokens {
    fn to_oxidate_tokens(&self, tokens: &mut proc_macro2::TokenStream);
}

pub trait ToTokenTree {
    fn to_tt(&self) -> proc_macro2::TokenTree;
}

impl<T> ToOxidateTokens for T
where
    T: ToTokenTree,
{
    fn to_oxidate_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend([self.to_tt().clone()])
    }
}
