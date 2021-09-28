use derive_syn_parse::Parse;
use macroscope_utils::find_macroscope;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::ParseStream, parse_macro_input, punctuated::Punctuated, Ident, LitStr, Path, Token,
};

#[derive(Debug, Parse)]
struct UsingArgs {
    crate_name: CrateName,
    #[peek(Token![|])]
    extra: Option<UsingArgsExtra>,
}

#[derive(Debug, Parse)]
enum CrateName {
    #[peek_with(is_lit_str, name = "Literal")]
    Literal(LitStr),
    #[peek_with(is_path, name = "Path")]
    Path(Path),
}

impl CrateName {
    fn path(self) -> proc_macro2::TokenStream {
        match self {
            CrateName::Literal(literal) => {
                let id = Ident::new(&normalize(&literal.value()), Span::call_site());
                quote!(#id)
            }
            CrateName::Path(path) => quote!(#path),
        }
    }
}

fn is_lit_str(token: ParseStream) -> bool {
    token.peek(LitStr)
}

fn is_path(token: ParseStream) -> bool {
    token.peek(syn::Token![::]) || token.peek(Ident)
}

#[derive(Debug, Parse)]
struct UsingArgsExtra {
    pipe: Token![|],
    #[parse_terminated(parse_str)]
    crates: Punctuated<LitStr, Token![|]>,
}

fn parse_str(input: ParseStream<'_>) -> syn::parse::Result<LitStr> {
    input.parse()
}

#[proc_macro]
pub fn build_using(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let UsingArgs {
        crate_name,
        extra: _,
    } = parse_macro_input!(input as UsingArgs);

    let macroscope = find_macroscope();
    let crate_name = crate_name.path();

    // let extra = extra
    //     .into_iter()
    //     .flat_map(|UsingArgsExtra { crates, .. }| crates.into_pairs().map(|p| p.into_value()));

    // let found = #macroscope::find_crate(#crate_name, #extra);
    // let found = #macroscope::quote_crate(found);

    (quote! {
        macro_rules! using {
            ($($path:tt)*) => {{
                #macroscope::quote! { ::#crate_name::macros::crates::$($path)* }
            }}
        }
    })
    .into()

    // let using_fn = quote! {
    //     let found = #macroscope::found_crate!($crate_name)
    //         $(
    //             .or_else(|_| crate_name($crates))
    //         )*
    //         .unwrap_or_else(|err| {
    //             #macroscope::couldnt_find!(err, $crate_name $($crates)*)
    //         });

    //     let prefix = #macroscope::found_crate_prefix(found);

    //     proc_macro2::TokenStream::from(quote! { #prefix :: macros :: crates :: #quoted })
    // };
}

fn normalize(name: &str) -> String {
    name.replace("-", "_")
}
