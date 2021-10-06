use crate::tools::proc_macro2;
use crate::tools::quote::{quote, ToTokens};
use crate::tools::syn::{self, parse::Parse};

use crate::derive_parse::validate::Validate;

#[derive(Debug)]
pub enum OrderedChoice<T, U>
where
    T: Parse,
    U: Parse,
{
    Left(T),
    Right(U),
}

impl<T, U> Validate for OrderedChoice<T, U>
where
    T: Parse + Validate,
    U: Parse + Validate,
{
    fn validate(stream: &syn::parse::ParseStream) -> bool {
        T::validate(stream) || U::validate(stream)
    }
}

impl<T, U> ToTokens for OrderedChoice<T, U>
where
    T: Parse + ToTokens,
    U: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            OrderedChoice::Left(left) => tokens.extend(quote!(#left)),
            OrderedChoice::Right(right) => tokens.extend(quote!(#right)),
        }
    }
}

impl<T, U> Parse for OrderedChoice<T, U>
where
    T: Parse,
    U: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        try_parse!(input => T)
            .map(|left| Ok(OrderedChoice::Left(left)))
            .unwrap_or_else(|| U::parse(input).map(|right| OrderedChoice::Right(right)))
    }
}
