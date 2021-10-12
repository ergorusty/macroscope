pub mod assertions;
pub mod ast;
pub mod coerce;
pub mod derive_parse;
pub mod error;
pub mod gensym;
pub mod hygiene;
pub(crate) mod impl_prelude;
pub mod parse;
pub mod prelude;
pub mod wrap;

pub use self::{gensym::unique_ident, hygiene::dollar_crate};
pub use macroscope_macro::build_using;
pub use macroscope_utils::tools::quote::{quote, quote_spanned};
pub use macroscope_utils::{find_crate, quote_crate, tools, Tokens};

pub mod macros {
    pub use macroscope_utils::{expr, snippet, try_parse, ty};
}
