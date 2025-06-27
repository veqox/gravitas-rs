#[derive(Debug)]
pub enum Span {
    Ref { start: usize, len: usize },
    Owned { data: Box<[u8]> },
}

impl Span {
    pub fn len(&self) -> usize {
        match self {
            Span::Ref { len, .. } => *len,
            Span::Owned { data } => data.len(),
        }
    }
}
