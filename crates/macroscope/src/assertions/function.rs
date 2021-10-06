use crate::{
    assertions::{traits::AssertResult, utils::assert_some},
    ast::{Function, Identifier, Signature},
    error::assertions::Property,
    impl_prelude::*,
};

#[derive(Debug)]
pub struct AssertAsync;

impl AstAssertion<Function> for AssertAsync {
    fn assert(&self, node: &Function) -> AssertResult {
        self.assert(&node.signature())
    }
}

impl AstAssertion<Signature> for AssertAsync {
    fn assert(&self, node: &Signature) -> AssertResult {
        assert_some(&node.asyncness(), "async", node)
    }
}

#[derive(Debug)]
pub struct AssertIdent {
    property: Property,
    value: String,
    message: Option<String>,
}

impl AssertIdent {
    pub fn new(property: impl Into<Property>, value: impl Into<String>) -> AssertIdent {
        AssertIdent {
            property: property.into(),
            value: value.into(),
            message: None,
        }
    }
}

impl AstAssertion<Identifier> for AssertIdent {
    fn assert(&self, node: &Identifier) -> AssertResult {
        let node_name = node.to_string();

        if self.value == node_name {
            Ok(())
        } else {
            Err(MacroError::expected_property(self.property.clone())
                .to_be(self.value.clone())
                .but_was(node)
                .into())
        }
    }
}

#[derive(Debug)]
pub struct AssertName {
    name: String,
}

impl AssertName {
    pub fn new(name: impl Into<String>) -> AssertName {
        AssertName { name: name.into() }
    }
}

impl AstAssertion<Function> for AssertName {
    fn assert(&self, node: &Function) -> AssertResult {
        self.assert(&node.signature())
    }
}

impl AstAssertion<Signature> for AssertName {
    fn assert(&self, node: &Signature) -> AssertResult {
        AssertIdent::new("function name", &self.name).assert(&node.function_name())
    }
}
