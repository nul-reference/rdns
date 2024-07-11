use std::fmt::Formatter;

use nom::IResult;

use crate::domain_name::DomainName;

#[derive(Clone, Debug)]
pub struct Question {
    name: crate::domain_name::DomainName,
    ty: super::Type,
    class: super::Class,
}

impl Question {
    pub fn new(
        name: crate::domain_name::DomainName,
        question_type: super::Type,
        class: super::Class,
    ) -> Self {
        Self {
            name,
            ty: question_type,
            class,
        }
    }

    pub fn name(&self) -> &crate::domain_name::DomainName {
        &self.name
    }

    pub fn question_type(&self) -> super::Type {
        self.ty
    }

    pub fn class(&self) -> super::Class {
        self.class
    }
}

impl From<Question> for Vec<u8> {
    fn from(value: Question) -> Self {
        let mut bytes = Vec::with_capacity(512);

        bytes.extend_from_slice(Vec::from(value.name).as_slice());
        bytes.extend_from_slice(&(value.ty as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.class as u16).to_be_bytes());

        bytes
    }
}

impl From<&Question> for Vec<u8> {
    fn from(value: &Question) -> Self {
        let mut bytes = Vec::with_capacity(512);

        bytes.extend_from_slice(Vec::from(value.name.clone()).as_slice());
        bytes.extend_from_slice(&(value.ty as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.class as u16).to_be_bytes());

        bytes
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, self.ty, self.class)
    }
}

#[tracing::instrument(skip_all)]
pub fn parse<'p>(message: &'p [u8]) -> impl Fn(&'p [u8]) -> IResult<&'p [u8], Question> {
    move |i: &'p [u8]| {
        let (remaining, domain_name) = DomainName::parse(message)(i)?;
        let (remaining, qtype) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, qclass) = nom::number::streaming::be_u16(remaining)?;
        Ok((
            remaining,
            Question::new(
                domain_name,
                qtype
                    .try_into()
                    .expect("Couldn't parse sensible query type"),
                qclass
                    .try_into()
                    .expect("Couldn't parse sensible query class"),
            ),
        ))
    }
}
