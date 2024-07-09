use std::fmt::Formatter;

use crate::ConversionError;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Query = 0,
    InverseQuery = 1,
    ServerStatusReport = 2,
}

impl TryFrom<u8> for Opcode {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::InverseQuery),
            2 => Ok(Self::ServerStatusReport),
            _ => Err(Self::Error::OutOfRange),
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Query => write!(f, "Query"),
            Self::InverseQuery => write!(f, "Inverse Query"),
            Self::ServerStatusReport => write!(f, "Server Status Report"),
        }
    }
}
