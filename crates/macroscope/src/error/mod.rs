pub mod assertions;

use macroscope_utils::{Span, Tokens};

use crate::{
    coerce::compile_error::CompileError,
    error::assertions::{
        AssertionFailureExpected, AssertionFailureProperty, DiagnosticError, Property,
    },
    impl_prelude::*,
};

#[derive(Debug, Clone, thiserror::Error)]
#[error("{reason}")]
pub struct CustomCompileError {
    pub span: Span,
    pub reason: String,
}

impl CompileError for CustomCompileError {
    fn into_compile_error(self) -> Tokens {
        let Self { span, reason } = self;

        macro_error!(display(reason) spanned span)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{error}")]
pub struct MissingCrate {
    error: proc_macro_crate::Error,
}

impl From<proc_macro_crate::Error> for MissingCrate {
    fn from(error: proc_macro_crate::Error) -> Self {
        MissingCrate { error }
    }
}

impl CompileError for MissingCrate {
    fn into_compile_error(self) -> Tokens {
        self.to_compile_error()
    }
}

impl ToCompileError for MissingCrate {
    fn to_compile_error(&self) -> Tokens {
        match &self.error {
            proc_macro_crate::Error::NotFound(path) => macro_error!(format(
                "Could not find `Cargo.toml` in manifest dir: `{}`.",
                path.display()
            )),
            proc_macro_crate::Error::CargoManifestDirNotSet => {
                macro_error!("`CARGO_MANIFEST_DIR` env variable not set.")
            }
            proc_macro_crate::Error::CouldNotRead { path, .. } => {
                macro_error!(format("Could not read `{}`.", path.display()))
            }
            proc_macro_crate::Error::InvalidToml { source } => {
                macro_error!(format("Invalid Cargo.toml: {}", source))
            }
            proc_macro_crate::Error::CrateNotFound { crate_name, path } => macro_error!(format(
                "Could not find `{}` in `dependencies` or `dev-dependencies` in `{}`!",
                crate_name,
                path.display()
            )),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MacroError {
    #[error("{0}")]
    SynError(#[from] crate::tools::syn::Error),
    #[error("{0}")]
    MissingCrate(#[from] MissingCrate),
    #[error("{0}")]
    Diagnostic(#[from] DiagnosticError),
    #[error("{0}")]
    Custom(#[from] CustomCompileError),
}

impl MacroError {
    pub fn expected(expected: impl Into<String>) -> AssertionFailureExpected {
        DiagnosticError::expected(expected)
    }

    pub fn expected_property(property: impl Into<Property>) -> AssertionFailureProperty {
        DiagnosticError::expected_property(property)
    }

    pub fn compile_error(span: impl Into<Span>, reason: impl Into<String>) -> MacroError {
        MacroError::Custom(CustomCompileError {
            span: span.into(),
            reason: reason.into(),
        })
    }

    pub fn to_compile_error(&self) -> Tokens {
        match self {
            MacroError::SynError(error) => error.to_compile_error(),
            MacroError::Custom(custom) => custom.to_compile_error(),
            MacroError::MissingCrate(missing) => missing.to_compile_error(),
            MacroError::Diagnostic(assertion) => assertion.to_compile_error(),
        }
    }
}

pub type MacroResult<T> = Result<T, MacroError>;
