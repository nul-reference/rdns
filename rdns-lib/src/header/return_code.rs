#[repr(u8)]
#[derive(Copy, Clone, Debug, derive_more::Display)]
pub enum ReturnCode {
    #[display(fmt = "No Error")]
    NoError = 0,
    #[display(fmt = "Format Error")]
    FormatError = 1,
    #[display(fmt = "Server Failure")]
    ServerFailure = 2,
    #[display(fmt = "Name Error")]
    NameError = 3,
    #[display(fmt = "Not Implemented")]
    NotImplemented = 4,
    #[display(fmt = "Refused")]
    Refused = 5,
    #[display(fmt = "Unknown Return Code ({}/{:01x})", _0, _0)]
    Unknown(u8),
}

impl From<u8> for ReturnCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFailure,
            3 => Self::NameError,
            4 => Self::NotImplemented,
            5 => Self::Refused,
            n => Self::Unknown(n),
        }
    }
}

impl From<ReturnCode> for u8 {
    fn from(value: ReturnCode) -> Self {
        match value {
            ReturnCode::NoError => 0,
            ReturnCode::FormatError => 1,
            ReturnCode::ServerFailure => 2,
            ReturnCode::NameError => 3,
            ReturnCode::NotImplemented => 4,
            ReturnCode::Refused => 5,
            ReturnCode::Unknown(n) => n,
        }
    }
}
