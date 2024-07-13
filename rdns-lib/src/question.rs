use nom::IResult;

use crate::domain_name::DomainName;

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{} {} {}", name, ty, class)]
pub struct Question {
    name: DomainName,
    ty: super::Type,
    class: super::Class,
}

impl Question {
    pub fn new(name: DomainName, question_type: super::Type, class: super::Class) -> Self {
        Self {
            name,
            ty: question_type,
            class,
        }
    }

    pub fn name(&self) -> &DomainName {
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
        bytes.extend_from_slice(&u16::from(value.ty).to_be_bytes());
        bytes.extend_from_slice(&u16::from(value.class).to_be_bytes());

        bytes
    }
}

impl From<&Question> for Vec<u8> {
    fn from(value: &Question) -> Self {
        let mut bytes = Vec::with_capacity(512);

        bytes.extend_from_slice(Vec::from(value.name.clone()).as_slice());
        bytes.extend_from_slice(&u16::from(value.ty).to_be_bytes());
        bytes.extend_from_slice(&u16::from(value.class).to_be_bytes());

        bytes
    }
}

#[tracing::instrument(skip_all)]
pub fn parse<'p>(message: &'p [u8]) -> impl Fn(&'p [u8]) -> IResult<&'p [u8], Question> {
    move |i: &'p [u8]| {
        let (remaining, domain_name) = DomainName::parse(message)(i)?;
        let (remaining, question_type) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, question_class) = nom::number::streaming::be_u16(remaining)?;
        Ok((
            remaining,
            Question::new(domain_name, question_type.into(), question_class.into()),
        ))
    }
}
