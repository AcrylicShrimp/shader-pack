#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(u32);

impl Symbol {
    pub(crate) const fn new(index: u32) -> Self {
        Self(index)
    }

    pub fn index(self) -> u32 {
        self.0
    }
}

impl From<Symbol> for u32 {
    fn from(value: Symbol) -> Self {
        value.index()
    }
}

impl From<u32> for Symbol {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
