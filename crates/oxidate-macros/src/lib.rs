use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use proc_macro_roids::DeriveInputExt;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

mod kw {
    syn::custom_keyword!(copy);
}

#[derive(Debug, Parse)]
struct AliasOptions {
    copy: Option<kw::copy>,
}

macro_rules! derive_alias {
    ($name:ident => $($token:tt)*) => {
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, item: TokenStream) -> TokenStream {
            let mut ast = parse_macro_input!(item as DeriveInput);
            let args = parse_macro_input!(args as AliasOptions);

            let mut derives = parse_quote!($($token)*);

            if args.copy.is_some() {
                derives = parse_quote!(#derives, Copy);
            }

            ast.append_derives(derives);

            TokenStream::from(quote! { #ast })
        }
    };
}

derive_alias!(data => Debug, Clone);
derive_alias!(key => Debug, Clone, Eq, PartialEq, Hash);
derive_alias!(base => Debug);
