use super::CodecError;

pub trait Encode<'a>: Sized {
    fn encode(self, encoder: &mut Encoder<'a>) -> Result<(), CodecError>;
}

pub struct Encoder<'a> {
    buf: &'a mut [u8; 4096],
    pos: usize,
}

impl<'a> Encoder<'a> {
    pub fn new(buf: &'a mut [u8; 4096]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), CodecError> {
        self.write_bytes(&value.to_be_bytes())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), CodecError> {
        self.write_bytes(&value.to_be_bytes())
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), CodecError> {
        self.write_bytes(&value.to_be_bytes())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), CodecError> {
        if bytes.len() > self.remaining() {
            return Err(CodecError::BufferOverflow(
                self.pos + bytes.len(),
                self.buf.len(),
            ));
        }

        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(&bytes);
        self.pos += bytes.len();

        Ok(())
    }

    pub fn copy_within(&mut self, start: usize, len: usize) -> Result<(), CodecError> {
        if start > self.pos {
            return Err(CodecError::BufferOverflow(start, self.buf.len()));
        }

        if len > self.remaining() {
            return Err(CodecError::BufferOverflow(self.pos + len, self.buf.len()));
        }

        self.buf.copy_within(start..len, self.pos);

        self.pos += len;

        Ok(())
    }
}
