pub use opcode::Opcode;
pub use return_code::ReturnCode;

mod opcode;
mod return_code;

#[derive(Copy, Clone, Debug)]
pub struct Header {
    id: u16,
    is_answer: bool,
    opcode: Opcode,
    authoritive_answer: bool,
    truncation: bool,
    recursion_desired: bool,
    recursion_available: bool,
    response_code: ReturnCode,
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
        response_code: ReturnCode,
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
            ReturnCode::NoError,
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

    pub fn response_code(&self) -> ReturnCode {
        self.response_code
    }
}

type HeaderFlags<'p> = nom::IResult<&'p [u8], (bool, u8, bool, bool, bool, bool, u8, u8)>;

fn parse_header_flags(input: &[u8]) -> HeaderFlags<'_> {
    use nom::bits::{
        bits,
        streaming::{bool, take},
    };
    use nom::sequence::tuple;

    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(tuple((
        bool,
        take(4_usize),
        bool,
        bool,
        bool,
        bool,
        take(3_usize),
        take(4_usize),
    )))(input)
}

#[tracing::instrument(skip_all)]
pub fn header_parser(i: &[u8]) -> nom::IResult<&[u8], Header> {
    let (i, id) = nom::number::streaming::be_u16(i)?;

    let (i, (qr, opcode, aa, tc, rd, ra, _zeros, return_code)) = parse_header_flags(i)?;
    let opcode = Opcode::from(opcode);
    let return_code = ReturnCode::from(return_code);

    Ok((
        i,
        Header::new(Some(id), qr, opcode, aa, tc, rd, ra, return_code),
    ))
}
