use std::fmt::Debug;

use crate::{error::assertions::DiagnosticError, impl_prelude::*};

pub type AssertResult = Result<(), DiagnosticError>;

pub trait AstAssertion<Node>: Debug + 'static
where
    Node: AstNode,
{
    fn assert(&self, node: &Node) -> AssertResult;
    fn assertion(self) -> Assertion<Node>
    where
        Self: Sized,
    {
        Assertion::new(Box::new(self), None)
    }

    fn message(self, message: impl Into<String>) -> Assertion<Node>
    where
        Self: Sized,
    {
        Assertion::new(Box::new(self), Some(message.into()))
    }
}

#[derive(Debug, new)]
pub struct Assertion<N>
where
    N: AstNode + 'static,
{
    assertion: Box<dyn AstAssertion<N>>,
    message: Option<String>,
}

impl<N> AstAssertion<N> for Assertion<N>
where
    N: AstNode + Debug,
{
    fn assertion(self) -> Assertion<N>
    where
        Self: Sized,
    {
        self
    }

    fn assert(&self, node: &N) -> AssertResult {
        self.assertion
            .assert(node)
            .map_err(|err| match &self.message {
                Some(message) => err.message(message),
                None => err,
            })
    }
}
