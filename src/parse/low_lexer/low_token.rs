use super::LowTokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LowToken {
    pub kind: LowTokenKind,
    pub len: u32,
}

impl LowToken {
    pub const fn new(kind: LowTokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}
