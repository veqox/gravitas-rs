use log::warn;

use crate::{
    class::Class,
    header::Header,
    proto::{
        CodecError,
        decoder::{Decode, Decoder},
    },
    question::Question,
    record::Record,
    r#type::Type,
};

#[derive(Debug)]
pub struct Packet<'a> {
    buf: &'a mut [u8; 4096],

    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

impl<'a> Packet<'a> {
    pub fn from_buf(buf: &'a mut [u8; 4096]) -> Result<Self, CodecError> {
        let decoder = &mut Decoder::new(buf);

        let header = Header::decode(decoder)?;

        let qdcount = decoder.read_u16()?;
        let ancount = decoder.read_u16()?;
        let nscount = decoder.read_u16()?;
        let arcount = decoder.read_u16()?;

        let mut questions = Vec::with_capacity(qdcount.into());
        for _ in 0..qdcount {
            questions.push(Question::decode(decoder)?);
        }

        let mut answers = Vec::with_capacity(ancount.into());
        for _ in 0..ancount {
            answers.push(Record::decode(decoder)?);
        }

        let mut authorities = Vec::with_capacity(nscount.into());
        for _ in 0..nscount {
            authorities.push(Record::decode(decoder)?);
        }

        let mut additionals = Vec::with_capacity(arcount.into());
        for _ in 0..arcount {
            additionals.push(Record::decode(decoder)?);
        }

        if decoder.remaining() > 0 {
            warn!("packet not read to end")
        }

        Ok(Self {
            buf,
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }

    pub fn add_answer(&mut self, answer: Record) {
        let answer_start =
            self.header.size() + self.questions.iter().map(|q| q.size()).sum::<usize>();
    }
}
