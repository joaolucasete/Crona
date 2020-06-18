use std::fmt;

#[derive(Copy,Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}..{}",self.start,self.end))
    }
}