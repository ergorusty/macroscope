use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Debug;

use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
    Token,
};

use crate::wrapper::HasParts;

pub trait Delimiter: Sized + Debug + Copy + 'static {
    fn parse<T>(input: ParseStream) -> syn::Result<(Self, T)>
    where
        T: Parse;

    fn wrap_content(self, tokens: &mut TokenStream, content: TokenStream);

    fn wrap<T>(self, content: T) -> TokenStream
    where
        T: Parse + Debug + ToTokens,
    {
        let delimited = Delimited {
            content,
            delimiter: self,
        };
        quote!(#delimited)
    }
}

impl Delimiter for Paren {
    fn parse<T>(input: ParseStream) -> syn::Result<(Self, T)>
    where
        T: Parse,
    {
        let content;

        let paren = parenthesized!(content in input);
        let inner = T::parse(&content)?;

        Ok((paren, inner))
    }

    fn wrap_content(self, tokens: &mut TokenStream, content: TokenStream) {
        self.surround(tokens, |tokens| tokens.extend(content))
    }
}

impl Delimiter for Brace {
    fn parse<T>(input: ParseStream) -> syn::Result<(Self, T)>
    where
        T: Parse,
    {
        let content;

        let paren = braced!(content in input);
        let inner = T::parse(&content)?;

        Ok((paren, inner))
    }

    fn wrap_content(self, tokens: &mut TokenStream, content: TokenStream) {
        self.surround(tokens, |tokens| tokens.extend(content))
    }
}

impl Delimiter for Bracket {
    fn parse<T>(input: ParseStream) -> syn::Result<(Self, T)>
    where
        T: Parse,
    {
        let content;

        let paren = bracketed!(content in input);
        let inner = T::parse(&content)?;

        Ok((paren, inner))
    }

    fn wrap_content(self, tokens: &mut TokenStream, content: TokenStream) {
        self.surround(tokens, |tokens| tokens.extend(content))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Pipes {
    left: Token![|],
    right: Token![|],
}

impl Delimiter for Pipes {
    fn parse<T>(input: ParseStream) -> syn::Result<(Self, T)>
    where
        T: Parse,
    {
        let left = <Token![|]>::parse(input)?;
        let body = T::parse(input)?;
        let right = <Token![|]>::parse(input)?;

        Ok((Pipes { left, right }, body))
    }

    fn wrap_content(self, tokens: &mut TokenStream, content: TokenStream) {
        let Self { left, right } = self;
        tokens.extend(quote!(#left #content #right));
    }
}

#[derive(Debug)]
pub struct Delimited<T, D>
where
    T: Parse + Debug,
    D: Delimiter + Debug,
{
    delimiter: D,
    content: T,
}

impl<T, D> ToTokens for Delimited<T, D>
where
    T: Parse + ToTokens + Debug,
    D: Delimiter + Debug,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { delimiter, content } = self;

        delimiter.wrap_content(tokens, quote!(#content));
    }
}

impl<'a, T, D> HasParts<'a> for Delimited<T, D>
where
    T: Parse + Debug + 'a,
    D: Delimiter + Debug + 'a,
{
    type IntoParts = (D, T);
    type AsParts = (&'a D, &'a T);

    fn into_parts(self) -> Self::IntoParts {
        (self.delimiter, self.content)
    }

    fn as_parts(&'a self) -> Self::AsParts {
        (&self.delimiter, &self.content)
    }
}

transparent_wrapper!(Delimited<T, D> where { T: Parse + Debug, D: Delimiter + Debug } => self.content as T);

impl<T, D> Parse for Delimited<T, D>
where
    T: Parse + Debug,
    D: Delimiter + Debug,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (delimiter, content) = D::parse(input)?;

        Ok(Delimited { delimiter, content })
    }
}

pub type Braced<T> = Delimited<T, Brace>;
pub type Parenthesized<T> = Delimited<T, Paren>;
pub type Bracketed<T> = Delimited<T, Bracket>;
pub type Piped<T> = Delimited<T, Pipes>;
