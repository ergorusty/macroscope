use crate::impl_prelude::*;
use macroscope_utils::tools::quote::IdentFragment;
use syn::parse::Parse;

ast_newtype!(Identifier {
    description: "identifier",
    inner: syn::Ident
});

impl IdentFragment for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Debug, Clone)]
pub struct LeafToken<T>
where
    T: syn::token::Token + ToTokens,
{
    inner: T,
}

impl<T> AstNewtype for LeafToken<T>
where
    T: syn::token::Token + ToTokens + std::fmt::Debug,
{
    type Inner = T;

    fn as_syn(&self) -> &Self::Inner {
        &self.inner
    }

    fn into_syn(self) -> Self::Inner {
        self.inner
    }
}

impl<T> From<T> for LeafToken<T>
where
    T: syn::token::Token + ToTokens,
{
    fn from(inner: T) -> Self {
        LeafToken { inner }
    }
}

impl<T> Parse for LeafToken<T>
where
    T: syn::token::Token + ToTokens,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse()
    }
}

impl<T> ToTokens for LeafToken<T>
where
    T: syn::token::Token + ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(tokens! { #self })
    }
}

impl<T> AstNode for LeafToken<T>
where
    T: syn::token::Token + ToTokens,
{
    type Inner = T;

    fn description(&self) -> String {
        self.to_token_stream().to_string()
    }

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
