#![forbid(clippy::unwrap_used)]
use std::fmt::Formatter;
use crate::question::Question;
use crate::resource_record::ResourceRecord;

pub mod header;
pub mod name;
mod parser;
pub mod question;
pub mod resource_record;

#[derive(Debug, Clone)]
pub struct Message {
    header: header::Header,
    questions: Vec<question::Question>,
    answers: Vec<resource_record::ResourceRecord>,
    authorities: Vec<resource_record::ResourceRecord>,
    additionals: Vec<resource_record::ResourceRecord>,
}

impl Message {
    pub fn new_query(recursion_desired: bool, questions: Vec<question::Question>) -> Self {
        Self {
            header: header::Header::new_question(header::Opcode::Query, recursion_desired),
            questions,
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn new_inverse_query(
        recursion_desired: bool,
        questions: Vec<resource_record::ResourceRecord>,
    ) -> Self {
        Self {
            header: header::Header::new_question(header::Opcode::InverseQuery, recursion_desired),
            questions: Vec::new(),
            answers: questions,
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn is_question(&self) -> bool {
        !self.header.is_query()
    }

    pub fn is_answer(&self) -> bool {
        self.header.is_query()
    }

    pub fn header(&self) -> &header::Header {
        &self.header
    }

    pub fn questions(&self) -> &[Question] {
        &self.questions[..]
    }

    pub fn answers(&self) -> &[ResourceRecord] {
        &self.answers[..]
    }

    pub fn authorities(&self) -> &[ResourceRecord] {
        &self.authorities[..]
    }

    pub fn additional_records(&self) -> &[ResourceRecord] {
        &self.additionals[..]
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(512);

        // Construct Header
        bytes.extend_from_slice(&value.header.id().to_be_bytes());
        let mut packed: u16 = 0;

        if value.header.is_query() {
            packed += 1 << 15;
        }
        packed += ((value.header.opcode() as u16) & 0x00_0F) << 11;
        if value.header.authoritive_answer() {
            packed += 1 << 10;
        }
        if value.header.truncation() {
            packed += 1 << 9;
        }
        if value.header.recursion_desired() {
            packed += 1 << 8;
        }
        if value.header.recursion_available() {
            packed += 1 << 7;
        }
        packed += (value.header.response_code() as u16) & 0x00_0F;

        bytes.extend_from_slice(&packed.to_be_bytes());
        bytes.extend_from_slice(&(value.questions.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.answers.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.authorities.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.additionals.len() as u16).to_be_bytes());

        // Construct questions
        for question in value.questions {
            bytes.extend_from_slice(Vec::from(question).as_slice());
        }

        bytes
    }
}

impl From<&[u8]> for Message {
    fn from(value: &[u8]) -> Self {
        let (_, ret) = parser::parse_message(value).expect("Couldn't parse structure of message");
        ret
    }
}

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
}

impl TryFrom<u16> for Type {
    type Error = ConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            3 => Ok(Self::MD),
            4 => Ok(Self::MF),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            7 => Ok(Self::MB),
            8 => Ok(Self::MG),
            9 => Ok(Self::MR),
            10 => Ok(Self::NULL),
            11 => Ok(Self::WKS),
            12 => Ok(Self::PTR),
            13 => Ok(Self::HINFO),
            14 => Ok(Self::MINFO),
            15 => Ok(Self::MX),
            16 => Ok(Self::TXT),
            252 => Ok(Self::AXFR),
            253 => Ok(Self::MAILB),
            254 => Ok(Self::MAILA),
            255 => Ok(Self::ALL),
            _ => Err(Self::Error::OutOfRange),
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
}

impl TryFrom<u16> for Class {
    type Error = ConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Internet),
            2 => Ok(Self::CSNET),
            3 => Ok(Self::Chaos),
            4 => Ok(Self::Hesiod),
            5 => Ok(Self::All),
            _ => Err(Self::Error::OutOfRange),
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
        }
    }
}

#[derive(Debug)]
pub enum ConversionError {
    OutOfRange,
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::OutOfRange => write!(f, "Out of range"),
        }
    }
}

impl std::error::Error for ConversionError {}
