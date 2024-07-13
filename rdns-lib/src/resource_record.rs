use std::net::Ipv4Addr;

use nom::IResult;

use crate::domain_name::DomainName;

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{} {} {} {} {}", name, ty, class, ttl, rdata)]
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

#[derive(Clone, Debug, derive_more::Display)]
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
    Null(Null),
    WKS(WKS),
    PTR(PTR),
    HostInfo(HostInfo),
    MInfo(MInfo),
    MX(MX),
    TXT(TXT),
    #[display(fmt = "<Unknown RR Class/Type {}/{}> {:?}", _0, _1, _2)]
    Unknown(super::Class, super::Type, Vec<u8>),
}
#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{} {}", cpu, os)]
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

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{} {}", responsible_mailbox, error_mailbox)]
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

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{} {}", preference, exchange)]
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

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{:?}", "bytes")]
pub struct Null {
    bytes: Vec<u8>,
}

impl Null {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
#[display(
    fmt = "{} {} {} {} {} {} {}",
    primary_source_domain,
    responsible_person_email,
    serial,
    refresh,
    retry,
    expire,
    minimum
)]
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

#[derive(Clone, Debug, derive_more::Display)]
pub struct TXT {
    text_data: String,
}

impl TXT {
    pub fn new(text_data: impl ToString) -> Self {
        Self {
            text_data: text_data.to_string(),
        }
    }

    pub fn data(&self) -> &str {
        &self.text_data
    }
}

#[derive(Clone, Debug, derive_more::Display)]
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

#[derive(Clone, Debug, derive_more::Display)]
#[display(fmt = "{} {} {}", address, protocol, r#"itertools::intersperse(ports.iter().map(|p| p.to_string()),  " ".to_string(),).reduce(|acc, s| format!("{acc}{s}")).unwrap_or("".to_string())"#)]
pub struct WKS {
    address: Ipv4Addr,
    protocol: Protocol,
    ports: Vec<u16>,
}

//noinspection ALL
#[derive(Copy, Clone, Debug, derive_more::Display, derive_more::From)]
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
    #[display(fmt = "BBN-RCC-MON")]
    BBNRCCMON = 10,
    #[display(fmt = "NVP-II")]
    NVP2 = 11,
    PUP = 12,
    ARGUS = 13,
    EMCON = 14,
    XNET = 15,
    CHAOS = 16,
    UDP = 17,
    MUX = 18,
    #[display(fmt = "DCN-MEAS")]
    DCNMEAS = 19,
    HMP = 20,
    PRM = 21,
    #[display(fmt = "XNS-IDP")]
    XNSIDP = 22,
    #[display(fmt = "TRUNK-1")]
    TRUNK1 = 23,
    #[display(fmt = "TRUNK-2")]
    TRUNK2 = 24,
    #[display(fmt = "LEAF-1")]
    LEAF1 = 25,
    #[display(fmt = "LEAF-2")]
    LEAF2 = 36,
    RDP = 27,
    IRTP = 28,
    #[display(fmt = "ISO-TP4")]
    ISOTP4 = 29,
    NETBLT = 30,
    #[display(fmt = "MFE-NSP")]
    MFENSP = 31,
    #[display(fmt = "MERIT-INP")]
    MERITINP = 32,
    SEP = 33,
    HostInternal = 61,
    CFPT = 62,
    LocalNetwork = 63,
    #[display(fmt = "SAT-EXPAK")]
    SATEXPAK = 64,
    #[display(fmt = "MIT-SUBNET")]
    MITSUBNET = 65,
    RDV = 66,
    IPPC = 67,
    DistributedFileSystem = 68,
    #[display(fmt = "SAT-MON")]
    SATMON = 69,
    IPCV = 71,
    #[display(fmt = "BR-SAT-MON")]
    BRSATMON = 76,
    #[display(fmt = "WB-MON")]
    WBMON = 78,
    #[display(fmt = "WB-EXPAK")]
    WBEXPAK = 79,
    #[display(fmt = "Unknown protocol {}", _0)]
    Unknown(u8),
}

#[tracing::instrument(skip_all)]
pub fn parse<'buf>(
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

        let ty: Type = ty.into();
        let class: Class = class.into();

        let record_data = match (class, ty) {
            (_, Type::CNAME) => {
                let (_, cname) = DomainName::parse(message)(data)?;
                RecordData::CName(CName::new(cname))
            }
            (_, Type::HINFO) => {
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
            (_, Type::MB) => {
                let (_, mail_agent_domain_name) = DomainName::parse(message)(data)?;
                RecordData::MB(MB::new(mail_agent_domain_name))
            }
            (_, Type::MD) => {
                let (_, mail_agent_domain_name) = DomainName::parse(message)(data)?;
                RecordData::MD(MD::new(mail_agent_domain_name))
            }
            (_, Type::MF) => {
                let (_, mail_agent_domain_name) = DomainName::parse(message)(data)?;
                RecordData::MF(MF::new(mail_agent_domain_name))
            }
            (_, Type::MG) => {
                let (_, mail_group_member_name) = DomainName::parse(message)(data)?;
                RecordData::MG(MG::new(mail_group_member_name))
            }
            (_, Type::MINFO) => {
                let (rem, responsible_mailbox) = DomainName::parse(message)(data)?;
                let (_, error_mailbox) = DomainName::parse(message)(rem)?;
                RecordData::MInfo(MInfo::new(responsible_mailbox, error_mailbox))
            }
            (_, Type::MR) => {
                let (_, new_name) = DomainName::parse(message)(data)?;
                RecordData::MR(MR::new(new_name))
            }
            (_, Type::MX) => {
                let (rem, preference) = nom::number::streaming::be_u16(data)?;
                let (_rem, domain_name) = DomainName::parse(message)(rem)?;
                RecordData::MX(MX::new(preference, domain_name))
            }
            (_, Type::NULL) => RecordData::Null(Null::new(data.to_vec())),
            (_, Type::NS) => {
                let (_, name_server) = DomainName::parse(message)(data)?;
                RecordData::NS(NS::new(name_server))
            }
            (_, Type::PTR) => {
                let (_, domain_name) = DomainName::parse(message)(data)?;
                RecordData::PTR(PTR::new(domain_name))
            }
            (_, Type::SOA) => {
                let (rem, domain_name) = DomainName::parse(message)(data)?;
                let (rem, responsible_mailbox) = DomainName::parse(message)(rem)?;
                let (rem, serial) = nom::number::complete::be_u32(rem)?;
                let (rem, refresh) = nom::number::complete::be_u32(rem)?;
                let (rem, retry) = nom::number::complete::be_u32(rem)?;
                let (rem, expire) = nom::number::complete::be_u32(rem)?;
                let (_, minimum) = nom::number::complete::be_u32(rem)?;

                RecordData::SOA(SOA::new(
                    domain_name,
                    responsible_mailbox,
                    serial,
                    refresh,
                    retry,
                    expire,
                    minimum,
                ))
            }
            (_, Type::TXT) => {
                let (_, text): (_, Vec<char>) = nom::multi::count(
                    nom::character::complete::anychar,
                    data_length as usize,
                )(data)?;
                let string: String = text.iter().collect();
                RecordData::TXT(TXT::new(string))
            }
            (Class::Internet, Type::A) => {
                let (_remaining, address_bytes) = nom::number::streaming::be_u32(data)?;
                RecordData::A(A::new(Ipv4Addr::from(address_bytes)))
            }
            (Class::Internet, Type::WKS) => {
                let (rem, address) =
                    nom::combinator::map(nom::number::complete::be_u32, |a| Ipv4Addr::from(a))(
                        data,
                    )?;
                let (rem, protocol) =
                    nom::combinator::map(nom::number::complete::be_u8, |proto| {
                        Protocol::from(proto)
                    })(rem)?;
                let ports: Vec<u16> = rem
                    .iter()
                    .map(|byte| {
                        vec![
                            byte & 0x80,
                            byte & 0x40,
                            byte & 0x20,
                            byte & 0x10,
                            byte & 0x08,
                            byte & 0x04,
                            byte & 0x02,
                            byte & 0x01,
                        ]
                    })
                    .flatten()
                    .enumerate()
                    .filter_map(|(index, bit)| {
                        if bit != 0 {
                            Some((index + 1) as u16)
                        } else {
                            None
                        }
                    })
                    .collect();
                RecordData::WKS(WKS::new(address, protocol, ports))
            }
            (class, ty) => RecordData::Unknown(class, ty, data.to_vec()),
        };

        Ok((
            remaining,
            ResourceRecord::new(name, ty, class, ttl, record_data),
        ))
    }
}
