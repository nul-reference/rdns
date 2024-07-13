#[repr(u8)]
#[derive(Copy, Clone, Debug, derive_more::Display)]
pub enum Opcode {
    #[display(fmt = "Query")]
    Query = 0,
    #[display(fmt = "InverseQuery")]
    InverseQuery = 1,
    #[display(fmt = "ServerStatusReport")]
    ServerStatusReport = 2,
    #[display(fmt = "Unknown Opcode ({}/{:01x}", _0, _0)]
    Unknown(u8),
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            1 => Self::InverseQuery,
            2 => Self::ServerStatusReport,
            n => Self::Unknown(n),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Query => 0,
            Opcode::InverseQuery => 1,
            Opcode::ServerStatusReport => 2,
            Opcode::Unknown(n) => n,
        }
    }
}
