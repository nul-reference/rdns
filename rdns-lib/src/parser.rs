use nom::IResult;

use crate::domain_name::DomainName;
use crate::question::Question;
use crate::resource_record::parse_resource_record;
use crate::{header, Message};

#[tracing::instrument(skip_all)]
fn parse_question<'p>(message: &'p [u8]) -> impl Fn(&'p [u8]) -> IResult<&'p [u8], Question> {
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

#[tracing::instrument(skip_all)]
pub(crate) fn parse_message(message: &[u8]) -> IResult<&[u8], Message> {
    // Parse full header
    let (remaining, header) = header::header_parser(message)?;
    let (remaining, question_count) = nom::number::streaming::be_u16(remaining)?;
    let (remaining, answer_count) = nom::number::streaming::be_u16(remaining)?;
    let (remaining, nameserver_count) = nom::number::streaming::be_u16(remaining)?;
    let (remaining, additional_record_count) = nom::number::streaming::be_u16(remaining)?;

    let (remaining, questions) =
        nom::multi::count(parse_question(message), question_count as usize)(remaining)?;
    let (remaining, answers) =
        nom::multi::count(parse_resource_record(message), answer_count as usize)(remaining)?;
    let (remaining, authorities) =
        nom::multi::count(parse_resource_record(message), nameserver_count as usize)(remaining)?;
    let (remaining, additionals) = nom::multi::count(
        parse_resource_record(message),
        additional_record_count as usize,
    )(remaining)?;

    Ok((
        remaining,
        Message {
            header,
            questions,
            answers,
            authorities,
            additionals,
        },
    ))
}
