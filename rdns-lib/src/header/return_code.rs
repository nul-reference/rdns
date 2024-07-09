use std::fmt::Formatter;

use crate::ConversionError;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ReturnCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl TryFrom<u8> for ReturnCode {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::FormatError),
            2 => Ok(Self::ServerFailure),
            3 => Ok(Self::NameError),
            4 => Ok(Self::NotImplemented),
            5 => Ok(Self::Refused),
            _ => Err(Self::Error::OutOfRange),
        }
    }
}

impl std::fmt::Display for ReturnCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoError => write!(f, "No Error"),
            Self::FormatError => write!(f, "Format Error"),
            Self::ServerFailure => write!(f, "Server Failure"),
            Self::NameError => write!(f, "Name Error"),
            Self::NotImplemented => write!(f, "Not Implemented"),
            Self::Refused => write!(f, "Refused"),
        }
    }
}
