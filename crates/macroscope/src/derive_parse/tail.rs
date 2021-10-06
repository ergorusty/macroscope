use crate::tools::proc_macro2;
use crate::tools::quote::{quote, ToTokens};
use crate::tools::syn::{
    self,
    parse::{discouraged::Speculative, Parse},
};

use crate::derive_parse::{validate::Validate, wrapper::HasParts};

#[derive(Debug)]
pub struct ListWithTail<T, U>
where
    T: Parse,
    U: Parse,
{
    pub head: Vec<T>,
    pub tail: U,
}

impl<'a, T, U> HasParts<'a> for ListWithTail<T, U>
where
    T: Parse + 'a,
    U: Parse + 'a,
{
    type IntoParts = (Vec<T>, U);
    type AsParts = (&'a [T], &'a U);

    fn into_parts(self) -> Self::IntoParts {
        (self.head, self.tail)
    }

    fn as_parts(&'a self) -> Self::AsParts {
        (&self.head, &self.tail)
    }
}

impl<T, U> Validate for ListWithTail<T, U>
where
    T: Parse + Validate,
    U: Parse + Validate,
{
    fn validate(stream: &syn::parse::ParseStream) -> bool {
        T::validate(stream) || U::validate(stream)
    }
}

impl<T, U> ToTokens for ListWithTail<T, U>
where
    T: Parse + ToTokens,
    U: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { head, tail } = self;

        tokens.extend(quote!(#(#head)* #tail))
    }
}

impl<T, U> Parse for ListWithTail<T, U>
where
    T: Parse,
    U: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut head = vec![];

        loop {
            let fork = input.fork();

            match T::parse(&fork) {
                Ok(part) => {
                    if fork.is_empty() {
                        let tail = U::parse(&input)?;
                        return Ok(ListWithTail { head, tail });
                    } else {
                        head.push(part);
                        input.advance_to(&fork);
                    }
                }
                Err(_) => {
                    let tail = U::parse(&input)?;
                    return Ok(ListWithTail { head, tail });
                }
            }
        }
    }
}
