use std::fmt::Formatter;

pub use error::Error;

pub mod domain_name;
mod error;
pub mod header;
pub mod message;
pub mod question;
pub mod resource_record;

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Type {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ALL = 255,
    Unknown(u16),
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            252 => Self::AXFR,
            253 => Self::MAILB,
            254 => Self::MAILA,
            255 => Self::ALL,
            x => Self::Unknown(x),
        }
    }
}

impl From<Type> for u16 {
    fn from(value: Type) -> Self {
        match value {
            Type::A => 1,
            Type::NS => 2,
            Type::MD => 3,
            Type::MF => 4,
            Type::CNAME => 5,
            Type::SOA => 6,
            Type::MB => 7,
            Type::MG => 8,
            Type::MR => 9,
            Type::NULL => 10,
            Type::WKS => 11,
            Type::PTR => 12,
            Type::HINFO => 13,
            Type::MINFO => 14,
            Type::MX => 15,
            Type::TXT => 16,
            Type::AXFR => 252,
            Type::MAILB => 253,
            Type::MAILA => 254,
            Type::ALL => 255,
            Type::Unknown(x) => x,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::NS => write!(f, "NS"),
            Self::MD => write!(f, "MD"),
            Self::MF => write!(f, "MF"),
            Self::CNAME => write!(f, "CNAME"),
            Self::SOA => write!(f, "SOA"),
            Self::MB => write!(f, "MB"),
            Self::MG => write!(f, "MG"),
            Self::MR => write!(f, "MR"),
            Self::NULL => write!(f, "NULL"),
            Self::WKS => write!(f, "WKS"),
            Self::PTR => write!(f, "PTR"),
            Self::HINFO => write!(f, "HINFO"),
            Self::MINFO => write!(f, "MINFO"),
            Self::MX => write!(f, "MX"),
            Self::TXT => write!(f, "TXT"),
            Self::AXFR => write!(f, "AXFR"),
            Self::MAILB => write!(f, "MAILB"),
            Self::MAILA => write!(f, "MAILA"),
            Self::ALL => write!(f, "ALL"),
            Self::Unknown(ty) => write!(f, "Unknown Type ({ty}/0x{ty:04x})"),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Class {
    Internet = 1,
    CSNET = 2,
    Chaos = 3,
    Hesiod = 4,
    All = 5,
    Unknown(u16),
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Internet,
            2 => Self::CSNET,
            3 => Self::Chaos,
            4 => Self::Hesiod,
            5 => Self::All,
            x => Self::Unknown(x),
        }
    }
}

impl From<Class> for u16 {
    fn from(value: Class) -> Self {
        match value {
            Class::Internet => 1,
            Class::CSNET => 2,
            Class::Chaos => 3,
            Class::Hesiod => 4,
            Class::All => 5,
            Class::Unknown(x) => x,
        }
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internet => write!(f, "IN"),
            Self::CSNET => write!(f, "CS"),
            Self::Chaos => write!(f, "CH"),
            Self::Hesiod => write!(f, "HS"),
            Self::All => write!(f, "* "),
            Self::Unknown(x) => write!(f, "Unknown Class ({x}/0x{x:04x}"),
        }
    }
}
