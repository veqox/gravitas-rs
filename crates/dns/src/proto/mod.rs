use std::{error::Error, fmt::Display};

pub mod decoder;
pub mod encoder;

#[derive(Debug)]
pub enum CodecError {
    BufferOverflow(usize, usize),
    FormatError,
}

impl Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CodecError {}
