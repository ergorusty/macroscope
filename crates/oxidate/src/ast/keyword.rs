use crate::{AstParse, ParseOutcome};
use oxidate_macros::key;

#[key]
pub struct Keyword {
    keyword: String,
}

#[allow(non_snake_case)]
pub fn Keyword(string: impl Into<String>) -> Keyword {
    Keyword {
        keyword: string.into(),
    }
}

impl AstParse for Keyword {
    fn try_parse(input: &mut crate::Lexer) -> ParseOutcome<Self> {
        let mut lookahead = input.begin();

        let word = match lookahead.as_any_word() {
            Some(word) => Keyword(word.to_string()),
            None => return lookahead.rollback(),
        };

        lookahead.commit(word)
    }

    fn check(input: &mut crate::Lexer) -> bool {
        let lookahead = input.lookahead();

        match lookahead.as_any_word() {
            Some(_) => true,
            None => false,
        }
    }
}
