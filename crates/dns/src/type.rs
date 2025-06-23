use log::warn;

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum Type {
    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    A,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    NS,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    CNAME,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    SOA,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    PTR,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    MX,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
    TXT,

    /// [RFC 3596](https://www.rfc-editor.org/rfc/rfc3596#section-2.1)
    AAAA,

    /// [RFC 6891](https://www.rfc-editor.org/rfc/rfc6891#section-6.1.1)
    OPT,

    Unknown(u16),
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            5 => Self::CNAME,
            6 => Self::SOA,
            12 => Self::PTR,
            15 => Self::MX,
            16 => Self::TXT,
            28 => Self::AAAA,
            41 => Self::OPT,
            _ => {
                warn!("unknown value for record type {}", value);
                Self::Unknown(value)
            }
        }
    }
}

impl From<Type> for u16 {
    fn from(val: Type) -> Self {
        match val {
            Type::A => 1,
            Type::NS => 2,
            Type::CNAME => 5,
            Type::SOA => 6,
            Type::PTR => 12,
            Type::MX => 15,
            Type::TXT => 16,
            Type::AAAA => 28,
            Type::OPT => 41,
            Type::Unknown(code) => code,
        }
    }
}
