use crate::{assertions::traits::AssertResult, impl_prelude::*};

pub(crate) fn assert_some<T>(
    option: &Option<T>,
    expected: impl Into<String>,
    actual: &impl AstNode,
) -> AssertResult {
    match option.as_ref() {
        Some(_) => Ok(()),
        None => Err(MacroError::expected(expected).actual(actual)),
    }
}
