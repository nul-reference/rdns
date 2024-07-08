use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Name {
    labels: Vec<String>,
}

impl Name {
    pub fn new(labels: Vec<String>) -> Self {
        Self { labels }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in self.labels.iter() {
            write!(f, "{l}.").unwrap()
        }
        write!(f, "")
    }
}

impl From<Name> for Vec<u8> {
    fn from(value: Name) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(256);

        for name in value.labels {
            bytes.push(name.len() as u8);
            bytes.extend_from_slice(name.as_bytes());
        }
        bytes.push(0);
        bytes
    }
}
