use std::{iter::FromIterator, marker::PhantomData};

use syn::punctuated::{IntoPairs as SynIntoPairs, Pair as SynPair};

use crate::tools::proc_macro2;
use crate::tools::quote::{quote, ToTokens};
use crate::tools::syn::{self, parse::Parse};
use crate::tools::syn::{parse::ParseStream, punctuated::Punctuated};

pub trait ParseFrom {
    type Item: Parse;

    fn parse_from(input: ParseStream<'_>) -> syn::Result<Self::Item>;
}

impl<T> ParseFrom for T
where
    T: Parse,
{
    type Item = T;

    fn parse_from(input: ParseStream<'_>) -> syn::Result<Self::Item> {
        T::parse(input)
    }
}

pub struct Pair<T, S> {
    inner: syn::punctuated::Pair<T, S>,
}

impl<T, S> From<syn::punctuated::Pair<T, S>> for Pair<T, S> {
    fn from(syn_pair: syn::punctuated::Pair<T, S>) -> Self {
        Pair { inner: syn_pair }
    }
}

transparent_wrapper!(
    Pair<T, S> where { T: Parse, S: Parse } =>
        self.inner as syn::punctuated::Pair<T, S>
);

impl<T, S> Pair<T, S>
where
    S: Clone,
{
    pub fn item(&self) -> &T {
        match &self.inner {
            SynPair::Punctuated(item, _) => item,
            SynPair::End(item) => item,
        }
    }

    pub fn into_item(self) -> T {
        self.inner.into_value()
    }

    pub fn into_separator(self) -> Option<S> {
        match self.inner {
            SynPair::Punctuated(_, punctuation) => Some(punctuation),
            SynPair::End(_) => None,
        }
    }

    pub fn flat_map<U>(&self, mapper: impl FnOnce(&T) -> Option<U>) -> Option<Pair<U, S>> {
        let inner = match &self.inner {
            SynPair::Punctuated(item, punctuation) => {
                SynPair::Punctuated(mapper(item)?, punctuation.clone())
            }
            SynPair::End(item) => SynPair::End(mapper(item)?),
        };

        Some(Pair { inner })
    }

    pub fn map<U>(&self, mapper: impl FnOnce(&T) -> U) -> Pair<U, S>
    where
        U: Parse,
    {
        let inner = match &self.inner {
            SynPair::Punctuated(item, punctuation) => {
                SynPair::Punctuated(mapper(item), punctuation.clone())
            }
            SynPair::End(item) => SynPair::End(mapper(item)),
        };

        Pair { inner }
    }
}

impl<T, S> std::fmt::Debug for Pair<T, S>
where
    T: Parse,
    S: Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Pair newtype}}")
    }
}

#[derive(Debug, Clone)]
pub struct Separated<T, S, P = T> {
    punctuated: Punctuated<T, S>,
    parse: PhantomData<P>,
}

impl<T, S, P> Separated<T, S, P>
where
    S: Clone,
{
    pub fn into_pairs(self) -> IntoSeparatedPairs<T, S> {
        IntoSeparatedPairs {
            pairs: self.punctuated.into_pairs(),
        }
    }

    pub fn map<U>(self, mapper: impl Fn(&T) -> U) -> Separated<U, S> {
        let mapped = self.punctuated.pairs().map(|pair| match pair {
            syn::punctuated::Pair::Punctuated(item, punctuation) => {
                syn::punctuated::Pair::Punctuated(mapper(item), punctuation.clone())
            }
            syn::punctuated::Pair::End(item) => syn::punctuated::Pair::End(mapper(item)),
        });

        Separated {
            punctuated: mapped.collect(),
            parse: PhantomData,
        }
    }
}

pub struct IntoSeparatedPairs<T, S> {
    pairs: SynIntoPairs<T, S>,
}

impl<T, S> Iterator for IntoSeparatedPairs<T, S> {
    type Item = Pair<T, S>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pairs.next()?;

        Some(Pair::from(next))
    }
}

impl<T, S, P> FromIterator<syn::punctuated::Pair<T, S>> for Separated<T, S, P> {
    fn from_iter<I: IntoIterator<Item = syn::punctuated::Pair<T, S>>>(iter: I) -> Self {
        let punctuated: Punctuated<T, S> = iter.into_iter().collect();

        Separated {
            punctuated,
            parse: PhantomData,
        }
    }
}

impl<T, S, P> FromIterator<Pair<T, S>> for Separated<T, S, P> {
    fn from_iter<I: IntoIterator<Item = Pair<T, S>>>(iter: I) -> Self {
        let punctuated = iter.into_iter().map(|i| i.inner).collect();

        Separated {
            punctuated,
            parse: PhantomData,
        }
    }
}

impl<T, S, P> From<Punctuated<T, S>> for Separated<T, S, P>
where
    P: ParseFrom,
{
    fn from(punctuated: Punctuated<T, S>) -> Self {
        Separated {
            punctuated,
            parse: PhantomData,
        }
    }
}

impl<T, S, P> ToTokens for Separated<T, S, P>
where
    T: ToTokens + Parse,
    S: ToTokens + Parse,
    P: ParseFrom<Item = T>,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let p = &self.punctuated;
        tokens.extend(quote!(#p))
    }
}

impl<T, S, P> Parse for Separated<T, S, P>
where
    T: Parse,
    S: Parse,
    P: ParseFrom<Item = T>,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let punctuated = Punctuated::parse_terminated_with(&input, |stream| P::parse_from(stream))?;

        Ok(Separated {
            punctuated,
            parse: PhantomData,
        })
    }
}
