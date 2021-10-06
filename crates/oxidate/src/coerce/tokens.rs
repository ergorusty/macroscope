use crate::{error::CustomCompileError, Tokens};

extern crate proc_macro;

pub trait ProcMacroTokens: Sized {
    fn into_std_tokens(self) -> proc_macro::TokenStream {
        let tokens2: proc_macro2::TokenStream = self.into_tokens().into();
        tokens2.into()
    }

    fn into_tokens(self) -> crate::Tokens;

    fn as_tokens(&self) -> crate::Tokens
    where
        Self: Clone,
    {
        self.clone().into_tokens()
    }
}

impl ProcMacroTokens for Tokens {
    fn into_std_tokens(self) -> proc_macro::TokenStream {
        self.into()
    }

    fn into_tokens(self) -> crate::Tokens {
        self
    }
}

impl ProcMacroTokens for proc_macro::TokenStream {
    fn into_std_tokens(self) -> proc_macro::TokenStream {
        self
    }

    fn into_tokens(self) -> crate::Tokens {
        self.into()
    }
}

impl ProcMacroTokens for CustomCompileError {
    fn into_std_tokens(self) -> proc_macro::TokenStream {
        self.into_tokens().into()
    }

    fn into_tokens(self) -> crate::Tokens {
        let reason = self.reason;
        crate::tokens!({ compile_error!(#reason) } spanned self.span)
    }
}
