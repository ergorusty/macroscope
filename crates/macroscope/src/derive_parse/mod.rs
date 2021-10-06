#[macro_use]
pub mod wrapper;

#[macro_use]
pub mod validate;

pub mod choice;
pub mod delimited;
pub mod optional;
pub mod pair;
pub mod separated;
pub mod tail;

use crate::tools::proc_macro2;
use crate::tools::syn;

use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use syn::Ident;

use macroscope_utils::tools::quote::quote;

#[macro_export]
macro_rules! found_crate {
    ($name:expr) => {
        match found {
            FoundCrate::Itself => quote! { crate },
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote! { #ident }
            }
        }
    };
}

pub fn found_crate_prefix(found: FoundCrate) -> proc_macro2::TokenStream {
    match found {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { #ident }
        }
    }
}

pub fn found_crate_path(
    found: FoundCrate,
    quoted: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let prefix = found_crate_prefix(found);

    (quote! {
        #prefix :: macros :: crates :: #quoted
    })
    .into()
}
