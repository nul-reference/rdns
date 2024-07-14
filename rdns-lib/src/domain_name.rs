use std::fmt::{Display, Formatter};

use itertools::Itertools;
use nom::IResult;

#[derive(Clone, Debug)]
pub struct DomainName {
    labels: Vec<String>,
}

impl DomainName {
    pub fn new(labels: Vec<String>) -> Self {
        Self { labels }
    }

    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    #[tracing::instrument]
    pub(crate) fn parse<'m>(
        full_message: &'m [u8],
    ) -> impl Fn(&'m [u8]) -> IResult<&'m [u8], Self, nom::error::Error<&[u8]>> {
        move |i: &'m [u8]| {
            let (i, (labels, terminator)) = nom::multi::many_till(
                Element::parse_label,
                nom::branch::alt((Element::parse_root, Element::parse_pointer)),
            )(i)?;

            match terminator {
                Element::Root => Ok((
                    i,
                    DomainName::new(labels.iter().map(|e| e.to_string()).collect()),
                )),
                // This shouldn't ever happen: Domains should either terminate with a Root (zero length)
                // byte, or with a pointer.
                Element::Label(_) => Err(nom::Err::Failure(nom::error::Error::new(
                    i,
                    nom::error::ErrorKind::Verify,
                ))),
                Element::Pointer(p) => {
                    let (_, domain_name) = Self::parse(full_message)(&full_message[p..])?;

                    let mut labels: Vec<String> = labels.iter().map(|e| e.to_string()).collect();
                    labels.append(&mut domain_name.labels().to_vec());

                    Ok((i, DomainName::new(labels)))
                }
            }
        }
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in self.labels.iter() {
            write!(f, "{l}.").unwrap()
        }
        write!(f, "")
    }
}

impl From<DomainName> for Vec<u8> {
    fn from(value: DomainName) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(256);

        for name in value.labels {
            bytes.push(name.len() as u8);
            bytes.extend_from_slice(name.as_bytes());
        }
        bytes.push(0);
        bytes
    }
}

impl std::str::FromStr for DomainName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let labels = s.split('.').map(|s| s.to_owned()).collect_vec();
        Ok(Self::new(labels))
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Element {
    Label(String),
    Root,
    Pointer(usize),
}

impl Element {
    fn parse_root(i: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::value(Self::Root, nom::bytes::complete::tag([0x00; 1]))(i)
    }

    fn parse_label(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, (_, length)) = nom::bits::<_, (u8, usize), nom::error::Error<(&[u8], usize)>, _, _>(
            nom::sequence::pair(
                nom::bits::complete::tag(0x00, 2_usize),
                nom::bits::complete::take(6_usize),
            ),
        )(i)?;
        let (i, label) =
            nom::combinator::map_res(nom::bytes::complete::take(length), |b: &[u8]| {
                String::from_utf8(b.to_vec())
            })(i)?;

        Ok((i, Self::Label(label)))
    }

    fn parse_pointer(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, (_, address)) = nom::bits::<_, (_, usize), nom::error::Error<(&[u8], usize)>, _, _>(
            nom::sequence::pair(
                nom::bits::complete::tag(0x3, 2_usize),
                nom::bits::complete::take(14_usize),
            ),
        )(i)?;
        Ok((i, Self::Pointer(address)))
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Label(l) => write!(f, "{l}"),
            Element::Root => write!(f, ""),
            Element::Pointer(p) => write!(f, "<ptr: {p:04X}>"),
        }
    }
}
