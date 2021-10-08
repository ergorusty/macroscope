pub mod string_pattern;

use oxidate_macros::key;

use crate::{AstParse, LexNode, Literal, Punctuation};

use self::string_pattern::StringPattern;
pub trait AstFilter<Output>
where
    Output: AstParse,
{
    fn assert(&self, node: &LexNode) -> Option<Output>;
}

pub trait Blanket {
    type Strategy;
}

impl<T, Output> AstFilter<Output> for T
where
    T: Blanket + Clone,
    Output: AstParse,
    amplify::Holder<T, <T as Blanket>::Strategy>: AstFilter<Output>,
{
    // Do this for each of sample trait methods:
    fn assert(&self, node: &LexNode) -> Option<Output> {
        amplify::Holder::new(self.clone()).assert(node)
    }
}

#[key]
pub struct BlanketForStringLike;

impl<T> AstFilter<Literal> for amplify::Holder<T, BlanketForStringLike>
where
    T: Blanket + AsRef<str>,
{
    fn assert(&self, _node: &LexNode) -> Option<Literal> {
        let _str = self.as_inner().as_ref();

        todo!()
    }
}

#[key]
pub struct LiteralFilter<P>
where
    P: StringPattern,
{
    pattern: Option<P>,
}

#[key]
pub struct AssertPunctuationStart;

impl<'a> AstFilter<Punctuation> for AssertPunctuationStart {
    fn assert(&self, node: &LexNode) -> Option<Punctuation> {
        let _punctuation = node.as_punctuation()?;

        todo!()
    }
}

// impl AstAssertion<LexNode<'_>> for StringPattern {
//     type Output = Literal;

//     fn assert(&self, node: &LexNode) -> Self::Output {
//         todo!()
//     }
// }
