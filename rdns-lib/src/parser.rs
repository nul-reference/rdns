use std::net::Ipv4Addr;

use nom::IResult;

use crate::{Class, Message, Type};
use crate::header::{Header, Opcode, RCode};
use crate::name::Name;
use crate::question::Question;
use crate::resource_record::ResourceRecord;

type HeaderFlags<'p> = IResult<&'p [u8], (bool, u8, bool, bool, bool, bool, u8, u8)>;

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
fn header_parser(i: &[u8]) -> IResult<&[u8], Header> {
    let (i, id) = nom::number::streaming::be_u16(i)?;

    let (i, (qr, opcode, aa, tc, rd, ra, _zeros, rcode)) = parse_header_flags(i)?;
    let opcode = Opcode::try_from(opcode).expect("Invalid opcode passed: {opcode:02X?}");
    let rcode = RCode::try_from(rcode).expect("Invalid rcode: {rcode:02X?}");

    Ok((i, Header::new(Some(id), qr, opcode, aa, tc, rd, ra, rcode)))
}

#[tracing::instrument(skip_all)]
fn parse_domain_name_root(i: &[u8]) -> IResult<&[u8], String> {
    nom::combinator::map(nom::bytes::streaming::tag([0x00_u8]), |_byte: &[u8]| {
        "".to_string()
    })(i)
}

#[tracing::instrument(skip_all)]
fn parse_length_nonpointer(i: &[u8]) -> IResult<&[u8], (bool, usize)> {
    let (r, (_tag, length)) =
        nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::sequence::tuple((
            nom::bits::streaming::tag(0x00, 2_usize),
            nom::bits::streaming::take(6_usize),
        )))(i)?;
    Ok((r, (false, length)))
}

#[tracing::instrument(skip_all)]
fn parse_length_pointer(i: &[u8]) -> IResult<&[u8], (bool, usize)> {
    let (r, (_tag, pointer)) =
        nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::sequence::tuple((
            nom::bits::streaming::tag(0x03, 2_usize),
            nom::bits::streaming::take(14_usize),
        )))(i)?;
    Ok((r, (true, pointer)))
}

#[tracing::instrument(skip_all)]
fn parse_domain_name_text(i: &[u8]) -> IResult<&[u8], String> {
    let (r, count) = nom::number::streaming::be_u8(i)?;
    debug_assert!(count <= 63);
    let (r, data) = nom::bytes::streaming::take(count)(r)?;
    let output = String::from_utf8_lossy(data).to_string();
    tracing::info!("String output is {output} and remaining data is {r:02X?}");
    Ok((r, String::from_utf8_lossy(data).to_string()))
}

#[tracing::instrument(skip_all)]
fn parse_domain_name_text_with_length(count: usize) -> impl Fn(&[u8]) -> IResult<&[u8], String> {
    debug_assert!(count <= 63);
    move |i: &[u8]| {
        let (r, data) = nom::bytes::streaming::take(count)(i)?;
        Ok((r, String::from_utf8_lossy(data).to_string()))
    }
}

#[tracing::instrument(skip_all)]
fn parse_domain_name<'p>(
    packet: &'p [u8],
) -> impl Fn(&'p [u8]) -> IResult<&'p [u8], Vec<String>> + 'p {
    move |i: &[u8]| {
        tracing::info!("Current parsing progress: {i:?}");
        let mut names: Vec<String> = Vec::new();
        let mut output_r = i;
        let mut gone_back: bool = false;
        let mut step_r = i;

        loop {
            let (r, (is_pointer, length_or_offset)) =
                nom::branch::alt((parse_length_pointer, parse_length_nonpointer))(step_r)
                    .expect("Didn't get pointer or nonpointer match");
            step_r = r;
            if length_or_offset == 0 {
                if !gone_back { output_r = &output_r[1..] };
                break;
            } else if is_pointer {
                tracing::info!("Parsing pointer with offset 0x{length_or_offset:04X}");
                let new_start = &packet[length_or_offset..];
                let (x, length) = nom::number::streaming::be_u8(new_start)?;
                let (x, name) = parse_domain_name_text_with_length(length as usize)(x)
                    .expect("Couldn't parse with pointer");
                names.push(name);
                output_r = &output_r[2..];
                gone_back = true;
                step_r = x;
            } else {
                tracing::info!("Parsing non-pointer with length 0x{length_or_offset}");
                let (x, name) = parse_domain_name_text_with_length(length_or_offset)(step_r)
                    .expect("Couldn't parse without pointer");
                names.push(name);
                if !gone_back {
                    output_r = &output_r[(length_or_offset + 1)..]
                }
                step_r = x;
            }
        }


        Ok((output_r, names))
    }
}

#[tracing::instrument(skip_all)]
fn parse_question<'p>(message: &'p [u8]) -> impl Fn(&'p [u8]) -> IResult<&'p [u8], Question> {
    move |i: &'p [u8]| {
        let (remaining, name) = parse_domain_name(message)(i)?;
        let (remaining, qtype) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, qclass) = nom::number::streaming::be_u16(remaining)?;
        Ok((
            remaining,
            Question::new(
                Name::new(name),
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
fn parse_resource_record<'buf>(
    message: &'buf [u8],
) -> impl Fn(&'buf [u8]) -> IResult<&'buf [u8], ResourceRecord> {
    move |i: &'buf [u8]| {
        let (remaining, name) = parse_domain_name(message)(i)?;
        tracing::info!("domain name parsed: {name:?}");
        let (remaining, ty) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, class) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, ttl) = nom::number::streaming::be_i32(remaining)?;
        let (remaining, rdlength) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, data) = nom::bytes::streaming::take(rdlength)(remaining)?;

        let ty: Type = ty.try_into().expect("Couldn't parse sensible RR type");
        let class: Class = class.try_into().expect("Couldn't parse sensible RR class");

        let rdata = match class {
            Class::Internet | Class::All => match ty {
                Type::A => {
                    let (_remaining, address_bytes) = nom::number::streaming::be_u32(data)?;
                    crate::resource_record::RData::A(crate::resource_record::A::new(
                        Ipv4Addr::from(address_bytes),
                    ))
                }
                Type::MX => {
                    let (rem, preference) = nom::number::streaming::be_u16(data)?;
                    let (_rem, domain_name) = parse_domain_name(message)(rem)?;
                    crate::resource_record::RData::MX(crate::resource_record::MX::new(
                        preference,
                        Name::new(domain_name),
                    ))
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };

        Ok((
            remaining,
            ResourceRecord::new(Name::new(name), ty, class, ttl, rdata),
        ))
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn parse_message(message: &[u8]) -> IResult<&[u8], Message> {
    // Parse full header
    let (remaining, header) = header_parser(message)?;
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
