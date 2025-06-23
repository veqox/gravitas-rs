use super::CodecError;

pub trait Decode<'a>: Sized {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, CodecError>;
}

pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), CodecError> {
        if pos > self.buf.len() {
            return Err(CodecError::BufferOverflow(self.pos, self.buf.len()));
        }

        self.pos = pos;

        Ok(())
    }

    pub fn read_u32(&mut self) -> Result<u32, CodecError> {
        Ok(u32::from_be_bytes([
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
            self.read_u8()?,
        ]))
    }

    pub fn read_u16(&mut self) -> Result<u16, CodecError> {
        Ok(u16::from_be_bytes([self.read_u8()?, self.read_u8()?]))
    }

    pub fn read_u8(&mut self) -> Result<u8, CodecError> {
        let value = self.peek_u8()?;
        self.pos += size_of::<u8>();
        Ok(value)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], CodecError> {
        if len > self.remaining() {
            return Err(CodecError::BufferOverflow(self.pos + len, self.buf.len()));
        }

        let bytes = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        Ok(bytes)
    }

    pub fn peek_u8(&self) -> Result<u8, CodecError> {
        if self.pos >= self.buf.len() {
            return Err(CodecError::BufferOverflow(self.pos, self.buf.len()));
        }

        Ok(self.buf[self.pos])
    }
}
