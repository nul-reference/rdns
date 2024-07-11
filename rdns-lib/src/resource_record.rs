use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;

use nom::IResult;

use crate::domain_name::DomainName;
use crate::{header, question, Message};

#[derive(Clone, Debug)]
pub struct ResourceRecord {
    name: DomainName,
    ty: super::Type,
    class: super::Class,
    ttl: i32,
    rdata: RecordData,
}

impl ResourceRecord {
    pub fn new(
        name: DomainName,
        ty: super::Type,
        class: super::Class,
        ttl: i32,
        rdata: RecordData,
    ) -> Self {
        Self {
            name,
            ty,
            class,
            ttl,
            rdata,
        }
    }
}

impl Display for ResourceRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.name, self.class, self.ttl, self.ty, self.rdata
        )
    }
}

#[derive(Clone, Debug)]
pub enum RecordData {
    A(A),
    NS(NS),
    MD(MD),
    MF(MF),
    CName(CName),
    SOA(SOA),
    MB(MB),
    MG(MG),
    MR(MR),
    WKS(WKS),
    PTR(PTR),
    HostInfo(HostInfo),
    MInfo(MInfo),
    MX(MX),
    TXT(TXT),
}

impl Display for RecordData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordData::A(a) => write!(f, "{}", a.address),
            RecordData::NS(_) => todo!(),
            RecordData::MD(_) => todo!(),
            RecordData::MF(_) => todo!(),
            RecordData::CName(_) => todo!(),
            RecordData::SOA(_) => todo!(),
            RecordData::MB(_) => todo!(),
            RecordData::MG(_) => todo!(),
            RecordData::MR(_) => todo!(),
            RecordData::WKS(_) => todo!(),
            RecordData::PTR(_) => todo!(),
            RecordData::HostInfo(_) => todo!(),
            RecordData::MInfo(_) => todo!(),
            RecordData::MX(mx) => write!(f, "{} {}", mx.preference, mx.exchange),
            RecordData::TXT(_) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CName {
    cname: DomainName,
}

impl CName {
    pub fn new(cname: DomainName) -> Self {
        Self { cname }
    }

    pub fn cname(&self) -> &DomainName {
        &self.cname
    }
}

#[derive(Clone, Debug)]
pub struct HostInfo {
    cpu: String,
    os: String,
}

impl HostInfo {
    pub fn new(cpu: String, os: String) -> Self {
        Self { cpu, os }
    }

    pub fn cpu(&self) -> &str {
        &self.cpu
    }

    pub fn os(&self) -> &str {
        &self.os
    }
}

#[derive(Clone, Debug)]
pub struct MB {
    mail_agent_domain_name: DomainName,
}

impl MB {
    pub fn new(mail_agent_domain_name: DomainName) -> Self {
        Self {
            mail_agent_domain_name,
        }
    }

    pub fn mail_agent_domain_name(&self) -> &DomainName {
        &self.mail_agent_domain_name
    }
}

#[derive(Clone, Debug)]
pub struct MD {
    mail_agent_domain_name: DomainName,
}

impl MD {
    pub fn new(mail_agent_domain_name: DomainName) -> Self {
        Self {
            mail_agent_domain_name,
        }
    }

    pub fn mail_agent_domain_name(&self) -> &DomainName {
        &self.mail_agent_domain_name
    }
}

#[derive(Clone, Debug)]
pub struct MF {
    mail_agent_domain_name: DomainName,
}

impl MF {
    pub fn new(mail_agent_domain_name: DomainName) -> Self {
        Self {
            mail_agent_domain_name,
        }
    }

    pub fn mail_agent_domain_name(&self) -> &DomainName {
        &self.mail_agent_domain_name
    }
}

#[derive(Clone, Debug)]
pub struct MG {
    mail_group_member_name: DomainName,
}

impl MG {
    pub fn new(mail_group_member_name: DomainName) -> Self {
        Self {
            mail_group_member_name,
        }
    }

    pub fn mail_group_member_name(&self) -> &DomainName {
        &self.mail_group_member_name
    }
}

#[derive(Clone, Debug)]
pub struct MInfo {
    responsible_mailbox: DomainName,
    error_mailbox: DomainName,
}

impl MInfo {
    pub fn new(responsible_mailbox: DomainName, error_mailbox: DomainName) -> Self {
        Self {
            responsible_mailbox,
            error_mailbox,
        }
    }

    pub fn responsible_mailbox(&self) -> &DomainName {
        &self.responsible_mailbox
    }

    pub fn error_mailbox(&self) -> &DomainName {
        &self.error_mailbox
    }
}

#[derive(Clone, Debug)]
pub struct MR {
    new_name: DomainName,
}

impl MR {
    pub fn new(new_name: DomainName) -> Self {
        Self { new_name }
    }

    pub fn new_name(&self) -> &DomainName {
        &self.new_name
    }
}

#[derive(Clone, Debug)]
pub struct MX {
    preference: u16,
    exchange: DomainName,
}

impl MX {
    pub fn new(preference: u16, exchange: DomainName) -> Self {
        Self {
            preference,
            exchange,
        }
    }

    pub fn preference(&self) -> u16 {
        self.preference
    }

    pub fn exchange(&self) -> &DomainName {
        &self.exchange
    }
}

#[derive(Clone, Debug)]
pub struct NULL {
    bytes: Vec<u8>,
}

impl NULL {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Clone, Debug)]
pub struct NS {
    domain_name: DomainName,
}

impl NS {
    pub fn new(domain_name: DomainName) -> Self {
        Self { domain_name }
    }

    pub fn domain_name(&self) -> &DomainName {
        &self.domain_name
    }
}

#[derive(Clone, Debug)]
pub struct PTR {
    pointer_domain_name: DomainName,
}

impl PTR {
    pub fn new(pointer_domain_name: DomainName) -> Self {
        Self {
            pointer_domain_name,
        }
    }

    pub fn domain_name(&self) -> &DomainName {
        &self.pointer_domain_name
    }
}

#[derive(Clone, Debug)]
pub struct SOA {
    primary_source_domain: DomainName,
    responsible_person_email: DomainName,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
}

impl SOA {
    pub fn new(
        primary_source_domain: DomainName,
        responsible_person_email: DomainName,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    ) -> Self {
        Self {
            primary_source_domain,
            responsible_person_email,
            serial,
            refresh,
            retry,
            expire,
            minimum,
        }
    }

    pub fn primary_source_domain(&self) -> &DomainName {
        &self.primary_source_domain
    }

    pub fn responsible_person_email(&self) -> &DomainName {
        &self.responsible_person_email
    }

    pub fn serial(&self) -> u32 {
        self.serial
    }

    pub fn refresh(&self) -> u32 {
        self.refresh
    }

    pub fn retry(&self) -> u32 {
        self.retry
    }

    pub fn expire(&self) -> u32 {
        self.expire
    }

    pub fn minimum(&self) -> u32 {
        self.minimum
    }
}

#[derive(Clone, Debug)]
pub struct TXT {
    text_data: String,
}

impl TXT {
    pub fn new(text_data: &str) -> Self {
        Self {
            text_data: text_data.to_string(),
        }
    }

    pub fn data(&self) -> &str {
        &self.text_data
    }
}

#[derive(Clone, Debug)]
pub struct A {
    address: Ipv4Addr,
}

impl A {
    pub fn new(address: Ipv4Addr) -> Self {
        Self { address }
    }

    pub fn address(&self) -> &Ipv4Addr {
        &self.address
    }
}

#[derive(Clone, Debug)]
pub struct WKS {
    address: Ipv4Addr,
    protocol: Protocol,
    ports: Vec<u16>,
}

impl WKS {
    pub fn new(address: Ipv4Addr, protocol: Protocol, ports: Vec<u16>) -> Self {
        Self {
            address,
            protocol,
            ports,
        }
    }

    pub fn address(&self) -> &Ipv4Addr {
        &self.address
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub fn ports(&self) -> &[u16] {
        &self.ports
    }
}

//noinspection ALL
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Protocol {
    ICMP = 1,
    IGMP = 2,
    GGP = 3,
    ST = 5,
    TCP = 6,
    UCL = 7,
    EGP = 8,
    IGP = 9,
    BBNRCCMON = 10,
    NVP2 = 11,
    PUP = 12,
    ARGUS = 13,
    EMCON = 14,
    XNET = 15,
    CHOS = 16,
    UDP = 17,
    MUX = 18,
    DCNMEAS = 19,
    HMP = 20,
    PRM = 21,
    XNSIDP = 22,
    TRUNK1 = 23,
    TRUNK2 = 24,
    LEAF1 = 25,
    LEAF2 = 36,
    RDP = 27,
    IRTP = 28,
    ISOTP4 = 29,
    NETBLT = 30,
    MFENSP = 31,
    MERITINP = 32,
    SEP = 33,
    HostInternal = 61,
    CFPT = 62,
    LocalNetwork = 63,
    SATEXPAK = 64,
    MITSUBNET = 65,
    RDV = 66,
    IPPC = 67,
    DistributedFileSystem = 68,
    SATMON = 69,
    IPCV = 71,
    BRSATMON = 76,
    WBMON = 78,
    WBEXPAK = 79,
}

#[tracing::instrument(skip_all)]
pub fn parse_resource_record<'buf>(
    message: &'buf [u8],
) -> impl Fn(&'buf [u8]) -> IResult<&'buf [u8], ResourceRecord> {
    use crate::{domain_name::DomainName, Class, Type};
    move |i: &'buf [u8]| {
        let (remaining, name) = DomainName::parse(message)(i)?;
        let (remaining, ty) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, class) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, ttl) = nom::number::streaming::be_i32(remaining)?;
        let (remaining, data_length) = nom::number::streaming::be_u16(remaining)?;
        let (remaining, data) = nom::bytes::streaming::take(data_length)(remaining)?;

        let ty: Type = ty.try_into().expect("Couldn't parse sensible RR type");
        let class: Class = class.try_into().expect("Couldn't parse sensible RR class");

        let record_data = match (class, ty) {
            (_, Type::CNAME) => {
                let (_, cname) = DomainName::parse(message)(data)?;
                RecordData::CName(CName::new(cname))
            }
            (_, Type::MINFO) => {
                let (_, (cpu, os)) = nom::sequence::pair(
                    nom::multi::length_count(
                        nom::number::streaming::be_u8,
                        nom::character::streaming::anychar,
                    ),
                    nom::multi::length_count(
                        nom::number::streaming::be_u8,
                        nom::character::streaming::anychar,
                    ),
                )(data)?;
                RecordData::HostInfo(HostInfo::new(
                    cpu.into_iter().collect(),
                    os.into_iter().collect(),
                ))
            }
            (_, Type::MX) => {
                let (rem, preference) = nom::number::streaming::be_u16(data)?;
                let (_rem, domain_name) = DomainName::parse(message)(rem)?;
                RecordData::MX(MX::new(preference, domain_name))
            }
            (Class::Internet, Type::A) => {
                let (_remaining, address_bytes) = nom::number::streaming::be_u32(data)?;
                RecordData::A(A::new(Ipv4Addr::from(address_bytes)))
            }
            _ => unimplemented!(),
        };

        Ok((
            remaining,
            ResourceRecord::new(name, ty, class, ttl, record_data),
        ))
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn parse(message: &[u8]) -> IResult<&[u8], Message> {
    // Parse full header
    let (remaining, header) = header::header_parser(message)?;
    let (remaining, question_count) = nom::number::streaming::be_u16(remaining)?;
    let (remaining, answer_count) = nom::number::streaming::be_u16(remaining)?;
    let (remaining, nameserver_count) = nom::number::streaming::be_u16(remaining)?;
    let (remaining, additional_record_count) = nom::number::streaming::be_u16(remaining)?;

    let (remaining, questions) =
        nom::multi::count(question::parse(message), question_count as usize)(remaining)?;
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
