use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DnsError {
    #[error("I/O: {0}")]
    Io(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Empty response")]
    EmptyResponse,

    #[error("Invalid response: {0}")]
    InvalidResponse(&'static str),
}

impl From<io::Error> for DnsError {
    fn from(value: io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl<E> From<nom::Err<E>> for DnsError {
    fn from(_value: nom::Err<E>) -> Self {
        Self::Parse("unknown error".to_string())
    }
}
