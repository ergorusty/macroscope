use std::error::Error;

use crate::{tests::err, Lexer};

use super::TestResult;

fn kw(source: impl AsRef<str>) -> Result<Lexer, Box<dyn Error>> {
    Ok(Lexer::source(source.as_ref())?)
}

macro_rules! keyword {
    ($keyword:tt) => {
        let mut lexer = kw(stringify!($keyword))?;

        lexer
            .begin()
            .as_word("abstract")
            .ok_or_else(|| err("Expected token abstract"))?;

        lexer.commit();

        assert!(
            lexer.is_eof(),
            "Expected lexer to be exhausted after matching token. Found {}",
            lexer.lookahead()
        );

        let lexer = kw(stringify!($keyword));
    };
}

#[test]
fn test_keyword() -> TestResult {
    keyword![abstract];

    Ok(())
}
