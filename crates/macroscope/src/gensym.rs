use crate::tools::{quote::format_ident, syn::Ident};
use nanoid::nanoid;

const SAFE_IDENT: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn unique_ident() -> Ident {
    let rand = nanoid!(32, &SAFE_IDENT);

    format_ident!("_{}", rand)
}
