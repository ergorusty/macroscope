//! This module *may* grow to eventually support all of syn's AST, but its
//! ambitions at the moment are smaller: provide common conveniences that we
//! already know we need.

#[macro_use]
pub(crate) mod newtype;

mod function;
mod leaf;
pub mod punctuated;
mod signature;
mod traits;

pub use self::function::Function;
pub use self::leaf::*;
pub use self::signature::*;
pub use self::traits::{AstNode, AstPart};
