use std::fmt::{Debug, Display};

use crate::Span;

use crate::tokens::ParseBuffer;
use crate::{error::MacroResult, tools::quote::ToTokens};

pub trait AstSpan {
    type Unspanned;

    fn span(&self) -> Span;
    fn unspanned(&self) -> &Self::Unspanned;
}

pub trait AstNode: Debug + Display + Clone + AstParse + ToTokens {
    fn span(&self) -> Span;
}

pub trait AstPeek: Sized {
    fn peek(input: &mut ParseBuffer) -> bool;
}

pub trait AstParse: Sized {
    /// try_parse returns Ok(None) if it was able to peek ahead
    /// non-destructively, and was able to determine that the stream didn't
    /// match the token. It returns Err if peeking succeeded, but then the parse
    /// failed later anyway.
    fn try_parse(input: &mut ParseBuffer) -> MacroResult<Option<Self>>;
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
