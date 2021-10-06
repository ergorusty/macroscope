use crate::impl_prelude::*;

pub trait AstNode: ToTokens + Spanned {
    type Inner: quote::ToTokens;

    fn tokens(&self) -> Tokens {
        self.to_token_stream()
    }

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;

    fn description(&self) -> String;

    fn without(mut self, child: impl AstPart<Self>) -> Self
    where
        Self: Sized,
    {
        child.remove_from_parent(self.inner_mut());
        self
    }

    fn with(mut self, child: impl AstPart<Self>) -> Self
    where
        Self: Sized,
    {
        child.add_to_parent(self.inner_mut());
        self
    }
}

pub trait AstPart<Parent>: AstNode
where
    Parent: AstNode,
{
    fn remove_from_parent(&self, parent: &mut Parent::Inner);

    fn add_to_parent(&self, parent: &mut Parent::Inner);
}

#[macro_export]
macro_rules! ast_part {
    ($name:ident in $parent:ty { add($with_parent:ident $(,$with_self:ident)?) $add:stmt; remove($without_parent:ident $(,$without_self:ident)?) $remove:stmt $(;)? }) => {
        impl AstPart<$parent> for $name {
            fn remove_from_parent(&self, parent: &mut <$parent as $crate::ast::AstNode>::Inner) {
                $(
                    let $without_self = self;
                )?

                let $without_parent = parent;
                $remove
            }

            fn add_to_parent(&self, parent: &mut <$parent as $crate::ast::AstNode>::Inner) {
                $(
                    let $with_self = self;
                )?

                let $with_parent = parent;
                $add
            }
        }
    };
}
