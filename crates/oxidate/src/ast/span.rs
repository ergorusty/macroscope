use std::ops::Deref;

use oxidate_macros::data;

use crate::{AstSpan, ToTokenTree};

#[data(copy)]
pub enum SpanData {
    Span(proc_macro2::Span),
    CallSite,
    MacroRules,
    EOF,
}

#[data(copy)]
pub struct Span {
    inner: SpanData,
}

impl Span {
    pub fn unspanned() -> Span {
        Span {
            inner: SpanData::CallSite,
        }
    }

    pub fn macro_rules() -> Span {
        Span {
            inner: SpanData::MacroRules,
        }
    }

    pub fn eof() -> Span {
        Span {
            inner: SpanData::EOF,
        }
    }
}

impl Into<proc_macro2::Span> for Span {
    fn into(self) -> proc_macro2::Span {
        match self.inner {
            SpanData::Span(span) => span,
            SpanData::CallSite => proc_macro2::Span::call_site(),
            SpanData::MacroRules => proc_macro2::Span::mixed_site(),
            SpanData::EOF => {
                // TODO: Think this through
                proc_macro2::Span::call_site()
            }
        }
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(span: proc_macro2::Span) -> Self {
        Span {
            inner: SpanData::Span(span),
        }
    }
}

#[data(copy)]
pub struct Spanned<T> {
    inner: T,
    span: Span,
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> AstSpan for Spanned<T> {
    type Unspanned = T;

    fn span(&self) -> Span {
        self.span
    }

    fn unspanned(&self) -> &Self::Unspanned {
        &self.inner
    }

    fn into_unspanned(self) -> Self::Unspanned {
        self.inner
    }
}

impl<T> ToTokenTree for Spanned<T>
where
    T: ToTokenTree,
{
    fn to_tt(&self) -> proc_macro2::TokenTree {
        let mut tt = self.inner.to_tt();
        tt.set_span(self.span.into());
        tt
    }
}

impl<T> Spanned<T> {
    pub fn wrap(inner: T, span: impl Into<Span>) -> Spanned<T> {
        Spanned {
            inner,
            span: span.into(),
        }
    }
}

pub trait Spannable {
    fn spanned(self, span: impl Into<Span>) -> Spanned<Self>
    where
        Self: Sized,
    {
        Spanned::wrap(self, span)
    }
}

impl<T> Spannable for T {}
