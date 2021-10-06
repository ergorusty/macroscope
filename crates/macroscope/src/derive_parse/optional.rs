use crate::tools::proc_macro2;
use crate::tools::quote::{quote, ToTokens};
use crate::tools::syn::{self, parse::Parse};

use crate::derive_parse::validate::Validate;

#[derive(Debug)]
pub enum Optional<T>
where
    T: Parse + Validate,
{
    Present(T),
    Missing,
}

impl<T> Into<Optional<T>> for Option<T>
where
    T: Parse + Validate,
{
    fn into(self) -> Optional<T> {
        match self {
            Some(present) => Optional::Present(present),
            None => Optional::Missing,
        }
    }
}

impl<T> Into<Option<T>> for Optional<T>
where
    T: Parse + Validate,
{
    fn into(self) -> Option<T> {
        match self {
            Optional::Present(present) => Some(present),
            Optional::Missing => None,
        }
    }
}

impl<T> ToTokens for Optional<T>
where
    T: Parse + ToTokens + Validate,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Optional::Present(present) => tokens.extend(quote!(#present)),
            Optional::Missing => {}
        }
    }
}

impl<T> Parse for Optional<T>
where
    T: Parse + Validate,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let is_valid = T::validate(&input);

        if is_valid {
            Ok(Optional::Present(T::parse(&input)?))
        } else {
            Ok(Optional::Missing)
        }
    }
}

#[derive(Debug)]
pub enum OptionalEnd<T>
where
    T: Parse,
{
    Present(T),
    Missing,
}

impl<T> ToTokens for OptionalEnd<T>
where
    T: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            OptionalEnd::Present(present) => tokens.extend(quote!(#present)),
            OptionalEnd::Missing => {}
        }
    }
}

impl<T> Parse for OptionalEnd<T>
where
    T: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(OptionalEnd::Missing)
        } else {
            let parsed = T::parse(input)?;
            Ok(OptionalEnd::Present(parsed))
        }
    }
}
