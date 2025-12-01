use std::num::ParseIntError;
use thiserror::Error;

// Parsing errors
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error("Unexpected length, expected {expect}, got {got}.")]
    MismatchedLength { expect: usize, got: usize },
    #[error("Unexpected key, got {0}.")]
    UnexpectedKey(String),
    #[error("{0}")]
    IntError(#[from] ParseIntError),
    #[error("Unexpected char, got: {0}")]
    WrongChar(char),
    #[error("Miscellaneous")]
    Misc,
}
