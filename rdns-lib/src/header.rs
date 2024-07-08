use std::fmt::Formatter;
use crate::ConversionError;

#[derive(Copy, Clone, Debug)]
pub struct Header {
    id: u16,
    is_answer: bool,
    opcode: Opcode,
    authoritive_answer: bool,
    truncation: bool,
    recursion_desired: bool,
    recursion_available: bool,
    response_code: RCode,
}

impl Header {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<u16>,
        is_answer: bool,
        opcode: Opcode,
        authoritive_answer: bool,
        truncation: bool,
        recursion_desired: bool,
        recursion_available: bool,
        response_code: RCode,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(rand::random),
            is_answer,
            opcode,
            authoritive_answer,
            truncation,
            recursion_desired,
            recursion_available,
            response_code,
        }
    }

    pub(crate) fn new_question(opcode: Opcode, recursion_desired: bool) -> Self {
        Self::new(
            None,
            false,
            opcode,
            false,
            false,
            recursion_desired,
            false,
            RCode::NoError,
        )
    }

    pub(crate) fn new_answer(
        id: u16,
        opcode: Opcode,
        authoritive_answer: bool,
        truncation: bool,
        recursion_desired: bool,
        recursion_available: bool,
        return_code: RCode,
    ) -> Self {
        Self::new(
            Some(id),
            false,
            opcode,
            authoritive_answer,
            truncation,
            recursion_desired,
            recursion_available,
            return_code,
        )
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub(crate) fn is_query(&self) -> bool {
        self.is_answer
    }

    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn authoritive_answer(&self) -> bool {
        self.authoritive_answer
    }

    pub fn truncation(&self) -> bool {
        self.truncation
    }

    pub fn recursion_desired(&self) -> bool {
        self.recursion_desired
    }

    pub fn recursion_available(&self) -> bool {
        self.recursion_available
    }

    pub fn response_code(&self) -> RCode {
        self.response_code
    }
}

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

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum RCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl TryFrom<u8> for RCode {
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

impl std::fmt::Display for RCode {
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
