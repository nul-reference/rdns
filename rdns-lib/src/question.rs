use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Question {
    name: crate::name::Name,
    ty: super::Type,
    class: super::Class,
}

impl Question {
    pub fn new(name: crate::name::Name, question_type: super::Type, class: super::Class) -> Self {
        Self {
            name,
            ty: question_type,
            class,
        }
    }

    pub fn name(&self) -> &crate::name::Name {
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