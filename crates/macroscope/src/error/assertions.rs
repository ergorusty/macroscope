use std::fmt::Display;

use macroscope_utils::tools::proc_macro_error::Level;

use crate::impl_prelude::*;

#[derive(Debug, Clone, thiserror::Error)]
#[error("{}", .message)]
pub struct DiagnosticError {
    message: String,
    span: Span,
    errors: Vec<(Span, String)>,
    helps: Vec<(Span, String)>,
    notes: Vec<(Span, String)>,
    infos: Vec<String>,
}

impl DiagnosticError {
    pub fn new(node: &impl AstNode, message: impl Into<String>) -> DiagnosticError {
        DiagnosticError {
            message: message.into(),
            span: node.span(),
            errors: vec![],
            helps: vec![],
            notes: vec![],
            infos: vec![],
        }
    }

    pub fn expected(expected: impl Into<String>) -> AssertionFailureExpected {
        AssertionFailureExpected {
            expected: expected.into(),
        }
    }

    pub fn expected_property(property: impl Into<Property>) -> AssertionFailureProperty {
        AssertionFailureProperty {
            property: property.into(),
        }
    }

    pub fn error(mut self, node: impl AstNode, message: impl Into<String>) -> Self {
        self.errors.push((node.span(), message.into()));
        self
    }

    pub fn help(mut self, node: impl AstNode, message: impl Into<String>) -> Self {
        self.helps.push((node.span(), message.into()));
        self
    }

    pub fn note(mut self, node: impl AstNode, message: impl Into<String>) -> Self {
        self.notes.push((node.span(), message.into()));
        self
    }

    pub fn info(mut self, message: impl Into<String>) -> Self {
        self.infos.push(message.into());
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        let original = self.message;
        self.message = message.into();
        self.info(original)
    }

    pub fn into_diagnostic(self) -> Diagnostic {
        let mut diagnostic = Diagnostic::spanned(self.span, Level::Error, self.message);

        for (span, message) in self.errors {
            diagnostic = diagnostic.span_error(span, message);
        }

        for (span, message) in self.helps {
            diagnostic = diagnostic.span_help(span, message);
        }

        for (span, message) in self.notes {
            diagnostic = diagnostic.span_note(span, message);
        }

        for message in self.infos {
            diagnostic = diagnostic.note(message);
        }

        diagnostic
    }
}

impl CompileError for DiagnosticError {
    fn into_compile_error(self) -> Tokens {
        self.into_diagnostic().to_token_stream()
    }
}

pub struct AssertionFailureExpected {
    expected: String,
}

impl AssertionFailureExpected {
    pub fn actual(self, node: &impl AstNode) -> DiagnosticError {
        DiagnosticError::new(node, format!("Expected {}", self.expected))
    }
}

#[derive(Debug, Clone)]
pub enum Property {
    Field(String),
    Node { kind: String, property: String },
    Other(String),
}

impl From<String> for Property {
    fn from(string: String) -> Self {
        Property::Other(string)
    }
}

impl From<&str> for Property {
    fn from(string: &str) -> Self {
        Property::Other(string.into())
    }
}

impl From<&String> for Property {
    fn from(string: &String) -> Self {
        Property::Other(string.into())
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Field(name) => write!(f, "field {}", name),
            Property::Other(name) => write!(f, "{}", name),
            Property::Node { kind, property } => write!(f, "{} for {}", property, kind),
        }
    }
}

#[derive(Debug)]
pub struct AssertionFailureProperty {
    property: Property,
}

impl AssertionFailureProperty {
    pub fn to_be(self, expected: impl Into<String>) -> AssertionFailurePropertyExpectation {
        AssertionFailurePropertyExpectation {
            property: self.property,
            expected: expected.into(),
        }
    }
}

#[derive(Debug)]
pub struct AssertionFailurePropertyExpectation {
    property: Property,
    expected: String,
}

impl AssertionFailurePropertyExpectation {
    pub fn but_was(self, node: &impl AstNode) -> DiagnosticError {
        DiagnosticError::new(
            node,
            format!("Expected {} to be {}", self.property, self.expected),
        )
    }
}
