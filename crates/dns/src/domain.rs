use crate::proto::CodecError;
use crate::proto::decoder::{Decode, Decoder};
use crate::proto::encoder::{Encode, Encoder};
use crate::span::Span;

#[derive(Debug)]
pub struct Domain {
    pub labels: Vec<Label>,
}

impl Domain {
    pub fn size(&self) -> usize {
        self.labels.iter().map(|l| l.data.len + 1).sum()
    }
}

#[derive(Debug)]
pub struct Label {
    pub data: Span,
}

impl From<Vec<Label>> for Domain {
    fn from(labels: Vec<Label>) -> Self {
        Self { labels }
    }
}

impl<'a> Decode<'a> for Vec<Label> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError> {
        let mut labels = vec![];

        loop {
            let len = decoder.read_u8()? as usize;

            match len {
                0 => break,
                len if len & 0xC0 == 0xC0 => {
                    let pointer: u16 = (((len & 0x3F) as u16) << 8) | decoder.read_u8()? as u16;

                    let pos = decoder.position();

                    decoder.seek(pointer.into())?;

                    labels.extend(Self::decode(decoder)?);

                    decoder.seek(pos)?;

                    break;
                }
                1..=63 => {
                    let label = Label {
                        data: Span {
                            len,
                            start: decoder.position(),
                        },
                    };

                    labels.push(label);

                    decoder.seek(decoder.position() + len)?;
                }
                _ => return Err(CodecError::FormatError)?,
            }
        }

        Ok(labels)
    }
}

impl<'a> Decode<'a> for Domain {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError> {
        Ok(Vec::<Label>::decode(decoder)?.into())
    }
}

impl<'a> Encode<'a> for Domain {
    fn encode(self, encoder: &mut Encoder<'a>) -> Result<(), CodecError> {
        for label in self.labels {
            encoder.write_u8(label.data.len as u8)?;
            encoder.copy_within(label.data.start, label.data.len)?;
        }
        encoder.write_u8(0)?;

        Ok(())
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self { labels: Vec::new() }
    }
}
