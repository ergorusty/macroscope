use crate::derive_parse::validate::Validate;
use crate::tools::proc_macro2;
use crate::tools::quote::{quote, ToTokens};
use crate::tools::syn::{self, parse::Parse};

#[derive(Debug)]
pub struct Prefixed<T, U>
where
    T: Parse,
    U: Parse,
{
    pub prefix: T,
    pub item: U,
}

transparent_wrapper!(Prefixed<T, U> where { T: Parse, U: Parse } => self.item as U);

impl<T, U> Validate for Prefixed<T, U>
where
    T: Parse + Validate,
    U: Parse,
{
    fn validate(stream: &syn::parse::ParseStream) -> bool {
        T::validate(stream)
    }
}

impl<T, U> ToTokens for Prefixed<T, U>
where
    T: Parse + ToTokens,
    U: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { prefix, item } = self;
        tokens.extend(quote!(#prefix #item))
    }
}

impl<T, U> Parse for Prefixed<T, U>
where
    T: Parse,
    U: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let prefix = T::parse(input)?;
        let item = U::parse(input)?;

        Ok(Prefixed { prefix, item })
    }
}

#[derive(Debug)]
pub struct Suffixed<T, U>
where
    T: Parse,
    U: Parse,
{
    pub item: T,
    pub suffix: U,
}

transparent_wrapper!(Suffixed<T, U> where { T: Parse, U: Parse } => self.item as T);

impl<T, U> Validate for Suffixed<T, U>
where
    T: Parse + Validate,
    U: Parse,
{
    fn validate(stream: &syn::parse::ParseStream) -> bool {
        T::validate(stream)
    }
}

impl<T, U> Parse for Suffixed<T, U>
where
    T: Parse,
    U: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = T::parse(input)?;
        let suffix = U::parse(input)?;

        Ok(Suffixed { item, suffix })
    }
}

impl<T, U> ToTokens for Suffixed<T, U>
where
    T: Parse + ToTokens,
    U: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { item, suffix } = self;
        tokens.extend(quote!(#item #suffix))
    }
}

#[derive(Debug)]
pub struct Pair<T, U>
where
    T: Parse,
    U: Parse,
{
    pub left: T,
    pub right: U,
}

impl<T, U> Validate for Pair<T, U>
where
    T: Parse + Validate,
    U: Parse,
{
    fn validate(stream: &syn::parse::ParseStream) -> bool {
        T::validate(stream)
    }
}

impl<T, U> ToTokens for Pair<T, U>
where
    T: Parse + ToTokens,
    U: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { left, right } = self;
        tokens.extend(quote!(#left #right))
    }
}

impl<T, U> Parse for Pair<T, U>
where
    T: Parse,
    U: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let left = T::parse(input)?;
        let right = U::parse(input)?;

        Ok(Pair { left, right })
    }
}
