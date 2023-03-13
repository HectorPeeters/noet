use crate::lexer::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(String, Option<Span>),
}
