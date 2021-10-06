use crate::{assertions::traits::Assertion, impl_prelude::*};
use macroscope_utils::{tools::quote::format_ident, Tokens};

use crate::tokens;

use crate::{error::MacroResult, parse_tokens};

pub struct WrapFn {
    input: Function,
    wrapper: Tokens,
    assertions: Vec<Assertion<Function>>,
}

impl ToTokens for WrapFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.emit() {
            Ok(output) => tokens.extend(output),
            Err(err) => tokens.extend(err.to_compile_error()),
        }
    }
}

impl WrapFn {
    pub fn parse(input: Tokens, wrapper: Tokens) -> MacroResult<WrapFn> {
        let input = parse_tokens!(input)?;

        Ok(WrapFn {
            input,
            wrapper,
            assertions: vec![],
        })
    }

    pub fn assert(mut self, assertion: impl AstAssertion<Function>) -> Self {
        self.assertions.push(assertion.assertion());
        self
    }

    pub fn emit(&self) -> MacroResult<Tokens> {
        let Self {
            input: func,
            wrapper,
            assertions,
        } = self;

        for assertion in assertions {
            assertion.assert(&func)?;
        }

        let attrs = func.attrs();
        let visibility = func.visibility();
        let fn_name = func.name();
        let fn_token = func.fn_token();
        let qualifiers = func.signature().qualifiers();
        let generics = func.signature().generic_params();
        let where_clause = func.signature().where_clause();
        let return_type = func.signature().return_type();
        let body = func.body();

        let inner_name = format_ident!("inner_{}", fn_name);

        let _outer = func.signature().without(Async::default());

        Ok(tokens! {
            #visibility #fn_token #fn_name #generics() #return_type #where_clause {
                #(#attrs)*
                #qualifiers #fn_token #inner_name #generics() #return_type {
                    #body
                }

                #wrapper(#inner_name())
            }
        })
    }
}
