#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Set the server to use for queries
    #[arg(short, long)]
    pub server: Option<String>,
    /// Use TCP instead of UDP for the query
    #[arg(long, default_value_t = false)]
    pub use_tcp: bool,
    /// Choose a class to query for
    #[arg(short, long, value_enum, default_value_t = Class::IN)]
    pub class: Class,
    /// Type of response to query for
    #[arg(short, long = "type", value_enum, default_value_t = Type::A)]
    pub ty: Type,
    /// The hostname to query for
    pub host_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
#[value(rename_all = "upper")]
pub(crate) enum Class {
    IN,
    CSNET,
    CHAOS,
    HESIOD,
    ANY,
}

impl From<Class> for rdns_lib::Class {
    fn from(value: Class) -> Self {
        match value {
            Class::IN => Self::Internet,
            Class::CSNET => Self::CSNET,
            Class::CHAOS => Self::Chaos,
            Class::HESIOD => Self::Hesiod,
            Class::ANY => Self::All,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
#[value(rename_all = "upper")]
pub(crate) enum Type {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    MX,
    TXT,
    ALL,
}

impl From<Type> for rdns_lib::Type {
    fn from(value: Type) -> Self {
        match value {
            Type::A => Self::A,
            Type::NS => Self::NS,
            Type::CNAME => Self::CNAME,
            Type::SOA => Self::SOA,
            Type::PTR => Self::PTR,
            Type::MX => Self::MX,
            Type::TXT => Self::TXT,
            Type::ALL => Self::ALL,
        }
    }
}
