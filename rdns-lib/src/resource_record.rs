use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;

#[derive(Clone, Debug)]
pub struct ResourceRecord {
    name: crate::name::Name,
    ty: super::Type,
    class: super::Class,
    ttl: i32,
    rdata: RData,
}

impl ResourceRecord {
    pub fn new(
        name: crate::name::Name,
        ty: super::Type,
        class: super::Class,
        ttl: i32,
        rdata: RData,
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
        write!(f, "{} {} {} {} {}", self.name, self.class, self.ttl, self.ty, self.rdata)
    }
}

#[derive(Clone, Debug)]
pub enum RData {
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

impl Display for RData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RData::A(a) => write!(f, "{}", a.address),
            RData::NS(_) => todo!(),
            RData::MD(_) => todo!(),
            RData::MF(_) => todo!(),
            RData::CName(_) => todo!(),
            RData::SOA(_) => todo!(),
            RData::MB(_) => todo!(),
            RData::MG(_) => todo!(),
            RData::MR(_) => todo!(),
            RData::WKS(_) => todo!(),
            RData::PTR(_) => todo!(),
            RData::HostInfo(_) => todo!(),
            RData::MInfo(_) => todo!(),
            RData::MX(mx) => write!(f, "{} {}", mx.preference, mx.exchange),
            RData::TXT(_) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CName {
    cname: crate::name::Name,
}

impl CName {
    pub fn new(cname: crate::name::Name) -> Self {
        Self { cname }
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
}

#[derive(Clone, Debug)]
pub struct MB {
    mail_agent_domain_name: crate::name::Name,
}

impl MB {
    pub fn new(mail_agent_domain_name: crate::name::Name) -> Self {
        Self {
            mail_agent_domain_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MD {
    mail_agent_domain_name: crate::name::Name,
}

impl MD {
    pub fn new(mail_agent_domain_name: crate::name::Name) -> Self {
        Self {
            mail_agent_domain_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MF {
    mail_agent_domain_name: crate::name::Name,
}

impl MF {
    pub fn new(mail_agent_domain_name: crate::name::Name) -> Self {
        Self {
            mail_agent_domain_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MG {
    mail_group_member_name: crate::name::Name,
}

impl MG {
    pub fn new(mail_group_member_name: crate::name::Name) -> Self {
        Self {
            mail_group_member_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MInfo {
    responsible_mailbox: crate::name::Name,
    error_mailbox: crate::name::Name,
}

impl MInfo {
    pub fn new(responsible_mailbox: crate::name::Name, error_mailbox: crate::name::Name) -> Self {
        Self {
            responsible_mailbox,
            error_mailbox,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MR {
    new_name: crate::name::Name,
}

impl MR {
    pub fn new(new_name: crate::name::Name) -> Self {
        Self { new_name }
    }
}

#[derive(Clone, Debug)]
pub struct MX {
    preference: u16,
    exchange: crate::name::Name,
}

impl MX {
    pub fn new(preference: u16, exchange: crate::name::Name) -> Self {
        Self {
            preference,
            exchange,
        }
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
}

#[derive(Clone, Debug)]
pub struct NS {
    nameserver_domain_name: crate::name::Name,
}

impl NS {
    pub fn new(nameserver_domain_name: crate::name::Name) -> Self {
        Self {
            nameserver_domain_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PTR {
    pointer_domain_name: crate::name::Name,
}

impl PTR {
    pub fn new(pointer_domain_name: crate::name::Name) -> Self {
        Self {
            pointer_domain_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SOA {
    primary_source_domain: crate::name::Name,
    responsible_person_email: crate::name::Name,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
}

impl SOA {
    pub fn new(
        primary_source_domain: crate::name::Name,
        responsible_person_email: crate::name::Name,
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
}

#[derive(Clone, Debug)]
pub struct A {
    address: Ipv4Addr,
}

impl A {
    pub fn new(address: Ipv4Addr) -> Self {
        Self { address }
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
}

//noinspection ALL
#[derive(Clone, Debug)]
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
