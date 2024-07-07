#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

impl LineCol {
    pub const fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}
