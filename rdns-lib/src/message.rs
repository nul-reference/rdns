use crate::{header, question, resource_record};

#[derive(Debug, Clone)]
pub struct Message {
    header: header::Header,
    questions: Vec<question::Question>,
    answers: Vec<resource_record::ResourceRecord>,
    authorities: Vec<resource_record::ResourceRecord>,
    additional_records: Vec<resource_record::ResourceRecord>,
}

impl Message {
    pub fn new_query(recursion_desired: bool, questions: Vec<question::Question>) -> Self {
        Self {
            header: header::Header::new_question(header::Opcode::Query, recursion_desired),
            questions,
            answers: Vec::new(),
            authorities: Vec::new(),
            additional_records: Vec::new(),
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
            additional_records: Vec::new(),
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

    pub fn questions(&self) -> &[question::Question] {
        &self.questions[..]
    }

    pub fn answers(&self) -> &[resource_record::ResourceRecord] {
        &self.answers[..]
    }

    pub fn authorities(&self) -> &[resource_record::ResourceRecord] {
        &self.authorities[..]
    }

    pub fn additional_records(&self) -> &[resource_record::ResourceRecord] {
        &self.additional_records[..]
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
        packed += ((u8::from(value.header.opcode()) as u16) & 0x00_0F) << 11;
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
        packed += (u8::from(value.header.response_code()) as u16) & 0x00_0F;

        bytes.extend_from_slice(&packed.to_be_bytes());
        bytes.extend_from_slice(&(value.questions.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.answers.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.authorities.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&(value.additional_records.len() as u16).to_be_bytes());

        // Construct questions
        for question in value.questions {
            bytes.extend_from_slice(Vec::from(question).as_slice());
        }

        bytes
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = crate::error::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse(value)
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn parse(message: &[u8]) -> Result<Message, crate::error::Error> {
    let (
        remaining,
        (header, question_count, answer_count, nameserver_count, additional_record_count),
    ) = nom::sequence::tuple((
        header::header_parser,
        nom::number::complete::be_u16,
        nom::number::complete::be_u16,
        nom::number::complete::be_u16,
        nom::number::complete::be_u16,
    ))(message)
    .map_err(|_| crate::error::Error::FormatError)?;

    let (_, (questions, answers, authorities, additional_records)) =
        nom::combinator::all_consuming(nom::sequence::tuple((
            nom::multi::count(question::parse(message), question_count as usize),
            nom::multi::count(resource_record::parse(message), answer_count as usize),
            nom::multi::count(resource_record::parse(message), nameserver_count as usize),
            nom::multi::count(
                resource_record::parse(message),
                additional_record_count as usize,
            ),
        )))(remaining)
        .map_err(|_| crate::error::Error::FormatError)?;

    Ok(Message {
        header,
        questions,
        answers,
        authorities,
        additional_records,
    })
}
