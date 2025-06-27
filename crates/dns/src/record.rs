use crate::{
    class::Class,
    domain::Domain,
    proto::{
        CodecError,
        decoder::{Decode, Decoder},
        encoder::{Encode, Encoder},
    },
    span::Span,
    r#type::Type,
};

#[derive(Debug)]
pub enum Record {
    Record {
        name: Domain,
        r#type: Type,
        class: Class,
        ttl: u32,
        data: RecordData,
    },
    OPTRecord {
        size: u16,
        flags: u32,
        options: Span,
    },
}

#[derive(Debug)]
pub enum RecordData {
    /// DNS A record field layout as per [RFC 1035 Section 3.4.1](https://www.rfc-editor.org/rfc/rfc1035#section-3.4.1)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    ADDRESS                    |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    A {
        address: Span,
    },

    /// DNS NS record field layout as per [RFC 1035 Section 3.3.11](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.11)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   NSDNAME                     /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    NS {
        nsdname: Domain,
    },

    /// DNS CNAME record field layout as per [RFC 1035 Section 3.3.1](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.1)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                     CNAME                     /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    CNAME {
        cname: Domain,
    },

    /// DNS SOA record field layout as per [RFC 1035 Section 3.3.13](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.13)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                     MNAME                     /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                     RNAME                     /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    SERIAL                     |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    REFRESH                    |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                     RETRY                     |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    EXPIRE                     |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    MINIMUM                    |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    SOA {
        mname: Domain,
        rname: Domain,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },

    /// DNS PTR record field layout as per [RFC 1035 Section 3.3.12](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.12)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   PTRDNAME                    /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    PTR {
        ptrdname: Domain,
    },

    /// DNS MX record field layout as per [RFC 1035 Section 3.3.9](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.9)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                  PREFERENCE                   |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   EXCHANGE                    /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    MX {
        preference: u16,
        exchange: Domain,
    },

    /// DNS TXT record field layout as per [RFC 1035 Section 3.3.14](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.14)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   TXT-DATA                    /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    TXT {
        text: Span,
    },

    /// DNS AAAA record field layout as per [RFC 3596 Section 2.2](https://www.rfc-editor.org/rfc/rfc3596#section-2.2)
    ///
    /// ```text
    ///   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    ADDRESS                    |
    /// |                                               |
    /// |                                               |
    /// |                                               |
    /// |                                               |
    /// |                                               |
    /// |                                               |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// ```
    AAAA {
        address: Span,
    },

    Unknown {
        data: Span,
    },
}

impl<'a> RecordData {
    fn size(&self) -> usize {
        match self {
            RecordData::A { address } => address.len(),
            RecordData::NS { nsdname } => nsdname.size(),
            RecordData::CNAME { cname } => cname.size(),
            RecordData::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => {
                mname.size()
                    + rname.size()
                    + size_of_val(serial)
                    + size_of_val(refresh)
                    + size_of_val(retry)
                    + size_of_val(expire)
                    + size_of_val(minimum)
            }
            RecordData::PTR { ptrdname } => ptrdname.size(),
            RecordData::MX {
                preference,
                exchange,
            } => size_of_val(preference) + exchange.size(),
            RecordData::TXT { text } => text.len(),
            RecordData::AAAA { address } => address.len(),
            RecordData::Unknown { data } => data.len(),
        }
    }
}

impl<'a> Decode<'a> for Record {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError> {
        let name = Domain::decode(decoder)?;
        let r#type = decoder.read_u16()?.into();
        let class = decoder.read_u16()?.into();
        let ttl = decoder.read_u32()?;
        let rd_length = decoder.read_u16()?.into();

        match r#type {
            Type::OPT => Ok(Self::OPTRecord {
                size: class,
                flags: ttl,
                options: {
                    let data = Span::Ref {
                        start: decoder.position(),
                        len: rd_length,
                    };

                    decoder.seek(decoder.position() + data.len())?;

                    data
                },
            }),
            _ => {
                let data = match r#type {
                    Type::A => RecordData::A {
                        address: {
                            let data = Span::Ref {
                                start: decoder.position(),
                                len: rd_length,
                            };

                            decoder.seek(decoder.position() + data.len())?;

                            data
                        },
                    },
                    Type::NS => RecordData::NS {
                        nsdname: Domain::decode(decoder)?,
                    },
                    Type::CNAME => RecordData::CNAME {
                        cname: Domain::decode(decoder)?,
                    },
                    Type::SOA => RecordData::SOA {
                        mname: Domain::decode(decoder)?,
                        rname: Domain::decode(decoder)?,
                        serial: decoder.read_u32()?,
                        refresh: decoder.read_u32()?,
                        retry: decoder.read_u32()?,
                        expire: decoder.read_u32()?,
                        minimum: decoder.read_u32()?,
                    },
                    Type::PTR => RecordData::PTR {
                        ptrdname: Domain::decode(decoder)?,
                    },
                    Type::MX => RecordData::MX {
                        preference: decoder.read_u16()?,
                        exchange: Domain::decode(decoder)?,
                    },
                    Type::TXT => RecordData::TXT {
                        text: {
                            let data = Span::Ref {
                                start: decoder.position(),
                                len: rd_length,
                            };

                            decoder.seek(decoder.position() + data.len())?;

                            data
                        },
                    },
                    Type::AAAA => RecordData::AAAA {
                        address: {
                            let data = Span::Ref {
                                start: decoder.position(),
                                len: rd_length,
                            };

                            decoder.seek(decoder.position() + data.len())?;

                            data
                        },
                    },
                    Type::OPT => unreachable!(),
                    Type::Unknown(_) => RecordData::Unknown {
                        data: {
                            let data = Span::Ref {
                                start: decoder.position(),
                                len: rd_length,
                            };

                            decoder.seek(decoder.position() + data.len())?;

                            data
                        },
                    },
                };

                Ok(Self::Record {
                    name,
                    r#type,
                    class: class.into(),
                    ttl,
                    data,
                })
            }
        }
    }
}

impl<'a> Encode<'a> for Record {
    fn encode(self, encoder: &mut Encoder<'a>) -> Result<(), CodecError> {
        match self {
            Record::Record {
                name,
                r#type,
                class,
                ttl,
                data,
            } => {
                name.encode(encoder)?;
                encoder.write_u16(r#type.into())?;
                encoder.write_u16(class.into())?;
                encoder.write_u32(ttl)?;
                encoder.write_u16(data.size() as u16)?;

                match data {
                    RecordData::A { address } => match address {
                        Span::Ref { start, len } => encoder.copy_within(start, len)?,
                        Span::Owned { data } => encoder.write_bytes(data.as_ref())?,
                    },
                    RecordData::NS { nsdname } => nsdname.encode(encoder)?,
                    RecordData::CNAME { cname } => cname.encode(encoder)?,
                    RecordData::SOA {
                        mname,
                        rname,
                        serial,
                        refresh,
                        retry,
                        expire,
                        minimum,
                    } => {
                        mname.encode(encoder)?;
                        rname.encode(encoder)?;
                        encoder.write_u32(serial)?;
                        encoder.write_u32(refresh)?;
                        encoder.write_u32(retry)?;
                        encoder.write_u32(expire)?;
                        encoder.write_u32(minimum)?;
                    }
                    RecordData::PTR { ptrdname } => ptrdname.encode(encoder)?,
                    RecordData::MX {
                        preference,
                        exchange,
                    } => {
                        encoder.write_u16(preference)?;
                        exchange.encode(encoder)?;
                    }
                    RecordData::TXT { text } => match text {
                        Span::Ref { start, len } => encoder.copy_within(start, len)?,
                        Span::Owned { data } => encoder.write_bytes(data.as_ref())?,
                    },
                    RecordData::AAAA { address } => match address {
                        Span::Ref { start, len } => encoder.copy_within(start, len)?,
                        Span::Owned { data } => encoder.write_bytes(data.as_ref())?,
                    },
                    RecordData::Unknown { data } => match data {
                        Span::Ref { start, len } => encoder.copy_within(start, len)?,
                        Span::Owned { data } => encoder.write_bytes(data.as_ref())?,
                    },
                }
            }
            Record::OPTRecord {
                size,
                flags,
                options,
            } => {
                Domain::default().encode(encoder)?;
                encoder.write_u16(Type::OPT.into())?;
                encoder.write_u16(size)?;
                encoder.write_u32(flags)?;
                encoder.write_u16(options.len() as u16)?;

                match options {
                    Span::Ref { start, len } => encoder.copy_within(start, len)?,
                    Span::Owned { data } => encoder.write_bytes(data.as_ref())?,
                };
            }
        };

        Ok(())
    }
}
