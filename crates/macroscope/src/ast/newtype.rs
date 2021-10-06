#[macro_export]
macro_rules! ast_newtype {
    ($ty:ident { description: $description:tt, inner: $inner:ty $(, is $token:tt $(,)?)? } no Parse) => {
        ast_newtype!($ty { description: |_| { $description }, inner: $inner } no Parse);
    };

    ($ty:ident { description: | $inner_param:tt | $description:block, inner: $inner:ty $(, is $token:tt $(,)?)? } no Parse) => {
        #[derive(Debug)]
        pub struct $ty {
            inner: $inner,
        }

        impl From<$inner> for $ty {
            fn from(inner: $inner) -> $ty {
                $ty { inner }
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inner.to_token_stream())
            }
        }

        impl $crate::ast::traits::AstNode for $ty {
            type Inner = $inner;

            fn description(&self) -> String {
                let $inner_param = &self.inner;

                $description.to_string()
            }

            fn inner(&self) -> &$inner {
                &self.inner
            }

            fn inner_mut(&mut self) -> &mut $inner {
                &mut self.inner
            }
        }

        impl $crate::tools::quote::ToTokens for $ty {
            fn to_tokens(&self, tokens: &mut $crate::Tokens) {
                self.inner.to_tokens(tokens)
            }
        }

        impl $crate::ast::newtype::AstNewtype for $ty {
            type Inner = $inner;

            fn as_syn(&self) -> &Self::Inner {
                &self.inner
            }

            fn into_syn(self) -> Self::Inner {
                self.inner
            }
        }
    };

    ($ty:ident { description: $description:tt, inner: $inner:ty $(, is $token:tt $(,)?)? }) => {
        ast_newtype!($ty { description: |_| { $description }, inner: $inner });
    };

    ($ty:ident { description: | $inner_param:tt | $description:tt, inner: $inner:ty $(, is $token:tt $(,)?)? }) => {
        ast_newtype!($ty { description: |$inner_param| $description, inner: $inner } no Parse);

        impl $crate::tools::syn::parse::Parse for $ty {
            fn parse(
                input: $crate::tools::syn::parse::ParseStream,
            ) -> $crate::tools::syn::Result<Self> {
                Ok(Self {
                    inner: input.parse()?,
                })
            }
        }
    };
}

#[macro_export]
macro_rules! token_newtype {
    ($ty:ident) => {
        impl $crate::derive_parse::validate::Validate for $ty {
            fn validate(stream: &$crate::tools::syn::parse::ParseStream) -> bool {
                <<Self as AstNode>::Inner as syn::token::Token>::peek(stream.cursor())
            }
        }

        impl Default for $ty {
            fn default() -> $ty {
                $ty {
                    inner: Default::default(),
                }
            }
        }
    };
}

use std::fmt::Debug;

pub trait AstNewtype: Debug {
    type Inner;

    fn as_syn(&self) -> &Self::Inner;
    fn into_syn(self) -> Self::Inner;
}
