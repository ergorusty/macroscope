use std::marker::PhantomData;

use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

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

#[derive(Debug)]
pub struct Separated<T, S, P = T>
where
    T: Parse,
    S: Parse,
    P: ParseFrom,
{
    punctuated: Punctuated<T, S>,
    parse: PhantomData<P>,
}

transparent_wrapper!(Separated<T, S, P> where { T: Parse, S: Parse, P: ParseFrom } => self.punctuated as Punctuated<T, S>);

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
