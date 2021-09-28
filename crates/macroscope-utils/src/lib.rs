mod macros;

pub mod tools {
    pub use {::proc_macro2, ::proc_macro_crate, ::quote, ::syn};
}

use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Ident, LitStr};

pub fn quote_crate(found: FoundCrate) -> proc_macro2::TokenStream {
    match found {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { #ident }
        }
    }
}

pub fn find_macroscope() -> proc_macro2::TokenStream {
    quote_crate(crate_name("macroscope").unwrap())
}

pub fn find_crate(first: LitStr /*, rest: impl Iterator<Item = LitStr> */) -> FoundCrate {
    match crate_name(&first.value()) {
        Ok(c) => return c,
        Err(err) => panic!("{}", err),
    }

    // let result = find(
    //     vec![first],
    //     rest,
    //     |accum: &mut Vec<LitStr>, lit, is_last| match crate_name(&lit.value()) {
    //         Ok(name) => MapResult::Found(name),
    //         Err(error) => {
    //             if !is_last {
    //                 accum.push(lit.clone());
    //             }
    //             MapResult::Failure(error)
    //         }
    //     },
    // );

    // match result {
    //     FindResult::Failure { error, accumulated } => {
    //         if accumulated.is_empty() {
    //             panic!("{}", error);
    //         } else {
    //             panic!(
    //                 "{}\nAlso tried: {}",
    //                 error,
    //                 itertools::join(accumulated.iter().map(|lit| lit.value()), ", ")
    //             );
    //         }
    //     }
    //     FindResult::Found(found) => found,
    // }
}

#[allow(unused)]
enum FindResult<T, E, A> {
    Found(T),
    Failure { error: E, accumulated: A },
}

#[allow(unused)]
enum MapResult<T, E> {
    Found(T),
    Failure(E),
}

#[allow(unused)]
fn find<T, U, A, E>(
    initial: A,
    iterator: impl Iterator<Item = T>,
    mapper: impl Fn(&mut A, &T, bool) -> MapResult<U, E>,
) -> FindResult<U, E, A> {
    let mut iter = iterator.peekable();
    let mut accum = initial;

    while let Some(item) = iter.next() {
        match mapper(&mut accum, &item, iter.peek().is_none()) {
            MapResult::Found(found) => return FindResult::Found(found),
            MapResult::Failure(error) if iter.peek().is_none() => {
                return FindResult::Failure {
                    error,
                    accumulated: accum,
                }
            }
            MapResult::Failure(_) => continue,
        };
    }

    unreachable!("loop should reach iter.peek() == None at some point")
}
