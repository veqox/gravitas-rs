use log::warn;

use crate::proto::{
    CodecError,
    decoder::{Decode, Decoder},
    encoder::{Encode, Encoder},
};

/// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
///
/// ```text
///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     FLAGS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: Flags,
}

impl Header {
    pub fn size(&self) -> usize {
        12
    }
}

impl<'a> Encode<'a> for Header {
    fn encode(self, encoder: &mut Encoder<'a>) -> Result<(), CodecError> {
        encoder.write_u16(self.id)?;
        self.flags.encode(encoder)?;

        Ok(())
    }
}

impl<'a> Decode<'a> for Header {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError> {
        Ok(Header {
            id: decoder.read_u16()?,
            flags: Flags::decode(decoder)?,
        })
    }
}

/// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
/// [RFC 2535](https://www.rfc-editor.org/rfc/rfc2535#section-6.1)
///
/// ```text
///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA| Z|AD|CD|   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Debug)]
pub struct Flags {
    pub message_type: MessageType,
    pub op_code: OpCode,
    pub authorative_answer: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub authentic_data: bool,
    pub checking_disabled: bool,
    pub response_code: RCode,
}

impl<'a> Decode<'a> for Flags {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError> {
        let flags = decoder.read_u16()?;

        if flags >> 6 & 0b1 != 0 {
            warn!("DNS query received with non-zero z flag");
        }

        Ok(Flags {
            message_type: match flags >> 15 & 0b1 == 1 {
                false => MessageType::Question,
                true => MessageType::Response,
            },
            op_code: ((flags >> 11 & 0b1111) as u8).into(),
            authorative_answer: flags >> 10 & 0b1 == 1,
            truncation: flags >> 9 & 0b1 == 1,
            recursion_desired: flags >> 8 & 0b1 == 1,
            recursion_available: flags >> 7 & 0b1 == 1,
            authentic_data: flags >> 6 & 0b1 == 1,
            checking_disabled: flags >> 6 & 0b1 == 1,
            response_code: ((flags >> 6 & 0b1) as u8).into(),
        })
    }
}

impl<'a> Encode<'a> for Flags {
    fn encode(self, encoder: &mut Encoder<'a>) -> Result<(), CodecError> {
        encoder.write_u16({
            let mut flags = 0;
            flags |= (self.message_type as u16) << 15;
            flags |= (u8::from(self.op_code) as u16 & 0b1111) << 11;
            flags |= (self.authentic_data as u16) << 10;
            flags |= (self.truncation as u16) << 9;
            flags |= (self.recursion_desired as u16) << 8;
            flags |= (self.recursion_available as u16) << 7;
            flags |= (self.authentic_data as u16) << 5;
            flags |= (self.checking_disabled as u16) << 4;
            flags |= u8::from(self.response_code) as u16;
            flags
        })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum MessageType {
    Question,
    Response,
}

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Query,
    Status,
    Notify,
    Update,
    Unknown(u8),
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            2 => Self::Status,
            4 => Self::Notify,
            5 => Self::Update,
            _ => {
                warn!("unknown value for opcode {}", value);
                Self::Unknown(value)
            }
        }
    }
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        match val {
            OpCode::Query => 0,
            OpCode::Status => 2,
            OpCode::Notify => 4,
            OpCode::Update => 5,
            OpCode::Unknown(x) => x,
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum RCode {
    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
    NoError,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
    FormatErr,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
    ServFail,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
    NXDomain,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
    NotImp,

    /// [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
    Refused,

    Unknown(u8),
}

impl From<u8> for RCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormatErr,
            2 => Self::ServFail,
            3 => Self::NXDomain,
            4 => Self::NotImp,
            5 => Self::Refused,
            other => Self::Unknown(other),
        }
    }
}

impl From<RCode> for u8 {
    fn from(val: RCode) -> Self {
        match val {
            RCode::NoError => 0,
            RCode::FormatErr => 1,
            RCode::ServFail => 2,
            RCode::NXDomain => 3,
            RCode::NotImp => 4,
            RCode::Refused => 5,
            RCode::Unknown(x) => x,
        }
    }
}
