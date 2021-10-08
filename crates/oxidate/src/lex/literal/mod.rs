pub mod literal_types;

#[cfg(test)]
mod tests;

use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use mown::Mown;
use oxidate_macros::data;

use crate::{AstParse, LiteralTrait, ParseOutcome, Spannable, Spanned, ToTokenTree};

use self::literal_types::{
    AnyLiteralSuffix, ByteLiteral, ByteStringLiteral, CharLiteral, FloatLiteral, IntegerLiteral,
    StringLiteral,
};

#[data]
#[derive(Eq, PartialEq)]
pub enum Literal {
    String(StringLiteral),
    ByteString(ByteStringLiteral),
    Byte(ByteLiteral),
    Char(CharLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
}

impl ToTokenTree for Literal {
    fn to_tt(&self) -> proc_macro2::TokenTree {
        let string = self.as_string();
        let literal = proc_macro2::Literal::from_str(&string).unwrap_or_else(|err| {
            panic!(
                "UNEXPECTED round-trip failure from literal \"{}\"\n\n== ERROR ==\n{}",
                string, err
            )
        });
        proc_macro2::TokenTree::Literal(literal)
    }
}

impl Literal {
    pub fn as_string(&self) -> Mown<str> {
        match self {
            Literal::String(v) => v.as_str(),
            Literal::ByteString(v) => v.as_str(),
            Literal::Byte(v) => v.as_str(),
            Literal::Char(v) => v.as_str(),
            Literal::Integer(v) => v.as_str(),
            Literal::Float(v) => v.as_str(),
        }
    }

    pub fn matches_suffix(&self, suffix: &str) -> bool {
        match self {
            Literal::String(v) => v.matches_suffix(suffix),
            Literal::ByteString(v) => v.matches_suffix(suffix),
            Literal::Byte(v) => v.matches_suffix(suffix),
            Literal::Char(v) => v.matches_suffix(suffix),
            Literal::Integer(v) => v.matches_suffix(suffix),
            Literal::Float(v) => v.matches_suffix(suffix),
        }
    }

    pub fn suffix(&self) -> AnyLiteralSuffix {
        match self {
            Literal::String(v) => v.generic_suffix(),
            Literal::ByteString(v) => v.generic_suffix(),
            Literal::Byte(v) => v.generic_suffix(),
            Literal::Char(v) => v.generic_suffix(),
            Literal::Integer(v) => v.generic_suffix(),
            Literal::Float(v) => v.generic_suffix(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Literal::String(string) => write!(f, "{}", string),
            Literal::ByteString(byte_string) => write!(f, "{}", byte_string),
            Literal::Byte(byte) => write!(f, "{}", byte),
            Literal::Char(char) => write!(f, "{}", char),
            Literal::Integer(int) => write!(f, "{}", int),
            Literal::Float(float) => write!(f, "{}", float),
        }
    }
}

impl AstParse for Literal {
    fn try_parse(lexer: &mut crate::Lexer) -> ParseOutcome<Self> {
        let mut lookahead = lexer.begin();

        let literal = match lookahead.as_literal() {
            Some(literal) => literal.clone(),
            None => return lookahead.rollback(),
        };

        lookahead.commit(literal)
    }

    fn check(input: &mut crate::Lexer) -> bool {
        let lookahead = input.lookahead();

        match lookahead.as_literal() {
            Some(_) => true,
            None => false,
        }
    }
}

impl AstParse for IntegerLiteral {
    fn try_parse(input: &mut crate::Lexer) -> ParseOutcome<Self> {
        let mut lookahead = input.begin();

        let int = match lookahead.as_literal() {
            Some(Literal::Integer(int)) => int.clone(),
            _ => return lookahead.rollback(),
        };

        lookahead.commit(int)
    }

    fn check(input: &mut crate::Lexer) -> bool {
        let lookahead = input.lookahead();

        match lookahead.as_literal() {
            Some(Literal::Integer(_)) => true,
            _ => false,
        }
    }
}

impl AstParse for FloatLiteral {
    fn try_parse(input: &mut crate::Lexer) -> ParseOutcome<Self> {
        let mut lookahead = input.begin();

        let float = match lookahead.as_literal() {
            Some(Literal::Float(float)) => float.clone(),
            _ => return lookahead.rollback(),
        };

        lookahead.commit(float)
    }

    fn check(lexer: &mut crate::Lexer) -> bool {
        lexer.lookahead().as_literal().is_some()
    }
}

impl From<proc_macro2::Literal> for Spanned<Literal> {
    fn from(literal: proc_macro2::Literal) -> Self {
        Literal::lex(&literal)
    }
}

impl From<&proc_macro2::Literal> for Spanned<Literal> {
    fn from(literal: &proc_macro2::Literal) -> Self {
        Literal::lex(literal)
    }
}

impl Literal {
    pub(crate) fn lex(literal: &proc_macro2::Literal) -> Spanned<Literal> {
        let string = literal.to_string();
        let span = literal.span();

        let lit = litrs::Literal::parse(string.clone()).unwrap_or_else(|err| {
            panic!(
                "Literals from proc_macro should always parse. Found unparseable literal: {:?}\n  === ERROR ===\n  {}\n\n",
                string, err
            )
        });

        let literal = match lit {
            litrs::Literal::Bool(_) => unreachable!(
                "proc_macro::Literal should not include booleans, so this should never happen"
            ),
            litrs::Literal::Integer(int) => Literal::Integer(int.into()),
            litrs::Literal::Float(float) => Literal::Float(float.into()),
            litrs::Literal::Char(char) => Literal::Char(char.into()),
            litrs::Literal::String(string) => Literal::String(string.into()),
            litrs::Literal::Byte(byte) => Literal::Byte(byte.into()),
            litrs::Literal::ByteString(byte_string) => Literal::ByteString(byte_string.into()),
        };

        literal.spanned(span)
    }
}
