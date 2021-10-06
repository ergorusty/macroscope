pub mod literal_types;
mod litrs_to_tt;

#[cfg(test)]
mod tests;

use std::fmt::{self, Debug, Display, Formatter};

use mown::Mown;
use oxidate_macros::data;

use crate::{AstSpan, LiteralTrait, Span, ToTokenTree};

use self::{
    literal_types::{
        AnyLiteralSuffix, ByteLiteral, ByteStringLiteral, CharLiteral, FloatLiteral,
        IntegerLiteral, StringLiteral,
    },
    litrs_to_tt::LitrsToTt,
};

#[data]
#[derive(Eq, PartialEq)]
pub enum UnspannedLiteral {
    String(StringLiteral),
    ByteString(ByteStringLiteral),
    Byte(ByteLiteral),
    Char(CharLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
}

impl UnspannedLiteral {
    pub fn as_string(&self) -> Mown<str> {
        match self {
            UnspannedLiteral::String(v) => v.as_str(),
            UnspannedLiteral::ByteString(v) => v.as_str(),
            UnspannedLiteral::Byte(v) => v.as_str(),
            UnspannedLiteral::Char(v) => v.as_str(),
            UnspannedLiteral::Integer(v) => v.as_str(),
            UnspannedLiteral::Float(v) => v.as_str(),
        }
    }

    pub fn matches_suffix(&self, suffix: &str) -> bool {
        match self {
            UnspannedLiteral::String(v) => v.matches_suffix(suffix),
            UnspannedLiteral::ByteString(v) => v.matches_suffix(suffix),
            UnspannedLiteral::Byte(v) => v.matches_suffix(suffix),
            UnspannedLiteral::Char(v) => v.matches_suffix(suffix),
            UnspannedLiteral::Integer(v) => v.matches_suffix(suffix),
            UnspannedLiteral::Float(v) => v.matches_suffix(suffix),
        }
    }

    pub fn suffix(&self) -> AnyLiteralSuffix {
        match self {
            UnspannedLiteral::String(v) => v.generic_suffix(),
            UnspannedLiteral::ByteString(v) => v.generic_suffix(),
            UnspannedLiteral::Byte(v) => v.generic_suffix(),
            UnspannedLiteral::Char(v) => v.generic_suffix(),
            UnspannedLiteral::Integer(v) => v.generic_suffix(),
            UnspannedLiteral::Float(v) => v.generic_suffix(),
        }
    }
}

impl Display for UnspannedLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            UnspannedLiteral::String(string) => write!(f, "{}", string),
            UnspannedLiteral::ByteString(byte_string) => write!(f, "{}", byte_string),
            UnspannedLiteral::Byte(byte) => write!(f, "{}", byte),
            UnspannedLiteral::Char(char) => write!(f, "{}", char),
            UnspannedLiteral::Integer(int) => write!(f, "{}", int),
            UnspannedLiteral::Float(float) => write!(f, "{}", float),
        }
    }
}

impl UnspannedLiteral {
    fn to_litrs(&self) -> litrs::Literal<String> {
        match self {
            UnspannedLiteral::String(string) => litrs::Literal::String(string.clone().into()),
            UnspannedLiteral::ByteString(byte_string) => {
                litrs::Literal::ByteString(byte_string.clone().into())
            }
            UnspannedLiteral::Byte(byte) => litrs::Literal::Byte(byte.clone().into()),
            UnspannedLiteral::Char(char) => litrs::Literal::Char(char.clone().into()),
            UnspannedLiteral::Integer(int) => litrs::Literal::Integer(int.clone().into()),
            UnspannedLiteral::Float(float) => litrs::Literal::Float(float.clone().into()),
        }
    }
}

#[data]
pub struct Literal {
    unspanned: UnspannedLiteral,
    span: Span,
}

impl Literal {
    pub fn matches_suffix(&self, suffix: &str) -> bool {
        self.unspanned.matches_suffix(suffix)
    }
}

impl AstSpan for Literal {
    type Unspanned = UnspannedLiteral;

    fn span(&self) -> crate::Span {
        self.span
    }

    fn unspanned(&self) -> &Self::Unspanned {
        &self.unspanned
    }
}

impl ToTokenTree for Literal {
    fn to_tt(&self) -> proc_macro2::TokenTree {
        let litrs_literal = self.unspanned.to_litrs();
        LitrsToTt::new(litrs_literal, self.span).into_tt()
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        Display::fmt(&self.unspanned, f)
    }
}

impl From<proc_macro2::Literal> for Literal {
    fn from(literal: proc_macro2::Literal) -> Self {
        Literal::lex(&literal)
    }
}

impl From<&proc_macro2::Literal> for Literal {
    fn from(literal: &proc_macro2::Literal) -> Self {
        Literal::lex(literal)
    }
}

impl Literal {
    pub(crate) fn lex(literal: &proc_macro2::Literal) -> Literal {
        let string = literal.to_string();
        let span = literal.span();

        let lit = litrs::Literal::parse(string.clone()).unwrap_or_else(|err| {
            panic!(
                "Literals from proc_macro should always parse. Found unparseable literal: {:?}\n  === ERROR ===\n  {}\n\n",
                string, err
            )
        });

        let raw = match lit {
            litrs::Literal::Bool(_) => unreachable!(
                "proc_macro::Literal should not include booleans, so this should never happen"
            ),
            litrs::Literal::Integer(int) => UnspannedLiteral::Integer(int.into()),
            litrs::Literal::Float(float) => UnspannedLiteral::Float(float.into()),
            litrs::Literal::Char(char) => UnspannedLiteral::Char(char.into()),
            litrs::Literal::String(string) => UnspannedLiteral::String(string.into()),
            litrs::Literal::Byte(byte) => UnspannedLiteral::Byte(byte.into()),
            litrs::Literal::ByteString(byte_string) => {
                UnspannedLiteral::ByteString(byte_string.into())
            }
        };

        Literal {
            unspanned: raw,
            span: span.into(),
        }
    }

    pub fn unspanned(&self) -> &UnspannedLiteral {
        &self.unspanned
    }
}
