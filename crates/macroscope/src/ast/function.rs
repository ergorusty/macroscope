use crate::impl_prelude::*;

ast_newtype!(Function {
    description: "function",
    inner: syn::ItemFn
});

impl Function {
    pub fn name(&self) -> Identifier {
        self.inner.sig.ident.clone().into()
    }

    pub fn visibility(&self) -> Visibility {
        Visibility::from(self.inner.vis.clone())
    }

    pub fn attrs(&self) -> Vec<Attribute> {
        self.inner.attrs.iter().cloned().map(|a| a.into()).collect()
    }

    pub fn fn_token(&self) -> SynToken![fn] {
        self.inner.sig.fn_token
    }

    pub fn signature(&self) -> ast::Signature {
        ast::Signature::from(self.as_syn().sig.clone())
    }

    pub fn body(&self) -> Block {
        Block::from(*self.inner.block.clone())
    }
}

ast_newtype!(Visibility {
    description: "visibility",
    inner: syn::Visibility
});
ast_newtype!(Attribute { description: "attribute", inner: syn::Attribute } no Parse);
ast_newtype!(Block {
    description: "block",
    inner: syn::Block
});
