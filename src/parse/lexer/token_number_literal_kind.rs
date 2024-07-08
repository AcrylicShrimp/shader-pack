#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenNumberLiteralKind {
    /// 0bN
    IntegerBinary,
    /// 0oN
    IntegerOctal,
    /// 0xN
    IntegerHexadecimal,
    /// N
    IntegerDecimal,
    /// - NeN
    /// - Ne+N
    /// - Ne-N
    /// - N.N
    /// - N.NeN
    /// - N.Ne+N
    /// - N.Ne-N
    Float,
}
