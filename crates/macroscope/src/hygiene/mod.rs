use crate::error::MacroResult;

use crate::impl_prelude::*;

pub fn dollar_crate(crate_name: impl AsRef<str>) -> MacroResult<Tokens> {
    use crate::tools::proc_macro_crate::{crate_name as find_crate, FoundCrate};

    let crate_name = crate_name.as_ref();

    match find_crate(crate_name) {
        Err(err) => Err(MacroError::MissingCrate(err.into())),
        Ok(FoundCrate::Itself) => Ok(tokens! { crate }),
        Ok(FoundCrate::Name(crate_name)) => {
            let id = syn::Ident::new(&crate_name, Span::call_site());
            Ok(tokens! { #id })
        }
    }
}

#[macro_export]
macro_rules! dollar_crate {
    ($crate_name:expr) => {
        match $crate::dollar_crate($crate_name) {
            Ok(crate_name) => crate_name,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}
