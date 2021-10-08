use std::error::Error;
use thiserror::Error;

mod keyword;
mod punctuation;

pub type TestResult = Result<(), Box<dyn Error>>;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct TestError {
    message: String,
}

pub fn err(string: impl Into<String>) -> TestError {
    TestError {
        message: string.into(),
    }
}
