use std::{f64::NAN, fmt::Debug};

use crate::{
    class::{self, Class},
    domain::Domain,
    proto::{
        CodecError,
        decoder::{Decode, Decoder},
        encoder::{Encode, Encoder},
    },
    r#type::Type,
};

/// DNS question field layout as per [RFC 1035 Section 4.1.2](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.2)
///
/// ```text
///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                     QNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QTYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QCLASS                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Debug)]
pub struct Question {
    pub name: Domain,
    pub r#type: Type,
    pub class: Class,
}

impl Question {
    pub fn size(&self) -> usize {
        self.name.size() + size_of::<Type>() + size_of::<Class>()
    }
}

impl<'a> Decode<'a> for Question {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError> {
        Ok(Question {
            name: Domain::decode(decoder)?,
            r#type: decoder.read_u16()?.into(),
            class: decoder.read_u16()?.into(),
        })
    }
}

impl<'a> Encode<'a> for Question {
    fn encode(self, encoder: &mut Encoder<'a>) -> Result<(), CodecError> {
        self.name.encode(encoder)?;
        encoder.write_u16(self.r#type.into())?;
        encoder.write_u16(self.class.into())?;

        Ok(())
    }
}
