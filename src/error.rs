use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("cannot process an empty field")]
    EmptyField,

    #[error("parse error: invalid character '{invalid_char}' at index {index}")]
    ParseError { invalid_char: char, index: usize },

    #[error("value overflowed during conversion: {0}")]
    OverflowError(String),

    #[error("scale ({0}) is too large or invalid for internal representation")]
    InvalidScale(usize),

    #[error("invalid field format string provided: {0}")]
    InvalidFormatString(String),

    #[error("character '{0}' is not supported by the specified encoding")]
    UnsupportedCharacter(char),
}
