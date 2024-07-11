use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    OutOfRange,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OutOfRange => write!(f, "Out of range"),
        }
    }
}

impl std::error::Error for Error {}
