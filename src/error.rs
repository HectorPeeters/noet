use std::fmt::{Display, Formatter};

use crate::lexer::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Parse(String, Option<Span>),
    Type(String, Option<Span>),
    Eval(String, Option<Span>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(message, _span) => write!(f, "Parse error: {message}"),
            Error::Type(message, _span) => write!(f, "Type error: {message}"),
            Error::Eval(message, _span) => write!(f, "Eval error: {message}"),
        }
    }
}
