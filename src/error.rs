use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TrailingChars,
    EOF,
    ExpectedNull,
    ExpectedBool,
    ExpectedInt,
    ExpectedString,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapComma,
    ExpectedMapColon,
    ExpectedMapEnd,
    ExpectedEnum,
    Syntax,

    Message(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            Error::EOF => "unexpected end of input",
            Error::TrailingChars => "trailing characters",
            Error::ExpectedNull => "expected null",
            Error::ExpectedBool => "expected boolean",
            Error::ExpectedInt => "expected integer",
            Error::ExpectedString => "expected string",
            Error::ExpectedArray => "expected array",
            Error::ExpectedArrayComma => "expected array comma",
            Error::ExpectedArrayEnd => "expected array end",
            Error::ExpectedMap => "expected map",
            Error::ExpectedMapComma => "expected map comma",
            Error::ExpectedMapColon => "expected map colon",
            Error::ExpectedMapEnd => "expected map end",
            Error::ExpectedEnum => "expected enum",
            Error::Syntax => "syntax error",

            Error::Message(msg) => msg,
        })
    }
}

impl std::error::Error for Error {}
