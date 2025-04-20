use super::TokenNumberLiteralKind;
use crate::symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Unknown {
        symbol: Symbol,
    },
    Whitespace {
        len: u32,
    },
    Comment {
        len: u32,
    }, // "#"
    OpenParen,    // "("
    CloseParen,   // ")"
    OpenBrace,    // "{"
    CloseBrace,   // "}"
    OpenBracket,  // "["
    CloseBracket, // "]"
    Dot,          // "."
    Comma,        // ","
    Colon,        // ":"
    Semicolon,    // ";"
    At,           // "@"
    Arrow,        // "->"
    // Assignment operators
    Assign,       // "="
    AssignAdd,    // "+="
    AssignSub,    // "-="
    AssignMul,    // "*="
    AssignDiv,    // "/="
    AssignMod,    // "%="
    AssignPow,    // "**="
    AssignShl,    // "<<="
    AssignShr,    // ">>="
    AssignBitOr,  // "|="
    AssignBitAnd, // "&="
    AssignBitXor, // "^="
    // Cmp operators
    Eq, // "=="
    Ne, // "!="
    Lt, // "<"
    Gt, // ">"
    Le, // "<="
    Ge, // ">="
    // Binary operators
    Add,    // "+"
    Sub,    // "-"
    Mul,    // "*"
    Div,    // "/"
    Mod,    // "%"
    Pow,    // "**"
    Shl,    // "<<"
    Shr,    // ">>"
    BitOr,  // "|"
    BitAnd, // "&"
    BitXor, // "^"
    LogOr,  // "||"
    LogAnd, // "&&"
    // Unary operators
    BitNot, // "~"
    LogNot, // "!"
    Id {
        symbol: Symbol,
    },
    BoolLiteral {
        content: Symbol,
    },
    NumberLiteral {
        kind: TokenNumberLiteralKind,
        /// The content of the number literal without the suffix.
        ///
        /// Example:
        /// - `123456i` -> `123456`
        content: Symbol,
        /// The suffix of the number literal.
        ///
        /// Example:
        /// - `123456i` -> `i`
        suffix: Option<Symbol>,
    },
    StringLiteral {
        /// The content of the string literal with the quotes.
        ///
        /// Example:
        /// - `"hello"` -> `"hello"`
        content: Symbol,
        /// The content of the string literal without the quotes.
        ///
        /// Example:
        /// - `"hello"` -> `hello`
        unquoted_content: Symbol,
        /// Indicates if the string literal is terminated correctly.
        /// This flag must be used to check if the string literal (or the entire program) is valid.
        ///
        /// Even without closing string literals, lexer can carry on parsing
        /// the rest of the file, by treating the rest of the file as being inside
        /// an unterminated string literal.
        /// But in that case, it's reasonable to treat the program as being invalid.
        ///
        /// Example:
        /// - `"baz"` -> `true`
        /// - `"bazz` -> `false`
        terminated: bool,
    },
}

impl TokenKind {
    pub fn len(self) -> u32 {
        match self {
            Self::Unknown { symbol } => symbol.to_str().len() as u32,
            Self::Whitespace { len } => len,
            Self::Comment { len } => len,
            Self::OpenParen => 1,
            Self::CloseParen => 1,
            Self::OpenBrace => 1,
            Self::CloseBrace => 1,
            Self::OpenBracket => 1,
            Self::CloseBracket => 1,
            Self::Dot => 1,
            Self::Comma => 1,
            Self::Colon => 1,
            Self::Semicolon => 1,
            Self::At => 1,
            Self::Arrow => 2,
            Self::Assign => 1,
            Self::AssignAdd => 2,
            Self::AssignSub => 2,
            Self::AssignMul => 2,
            Self::AssignDiv => 2,
            Self::AssignMod => 2,
            Self::AssignPow => 3,
            Self::AssignShl => 3,
            Self::AssignShr => 3,
            Self::AssignBitOr => 2,
            Self::AssignBitAnd => 2,
            Self::AssignBitXor => 2,
            Self::Eq => 2,
            Self::Ne => 2,
            Self::Lt => 1,
            Self::Gt => 1,
            Self::Le => 2,
            Self::Ge => 2,
            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 1,
            Self::Div => 1,
            Self::Mod => 1,
            Self::Pow => 2,
            Self::Shl => 2,
            Self::Shr => 2,
            Self::BitOr => 1,
            Self::BitAnd => 1,
            Self::BitXor => 1,
            Self::LogOr => 2,
            Self::LogAnd => 2,
            Self::BitNot => 1,
            Self::LogNot => 1,
            Self::Id { symbol } => symbol.to_str().len() as u32,
            Self::BoolLiteral { content } => content.to_str().len() as u32,
            Self::NumberLiteral {
                content, suffix, ..
            } => {
                let content_len = content.to_str().len();
                let suffix_len = suffix.map_or(0, |s| s.to_str().len());
                (content_len + suffix_len) as u32
            }
            Self::StringLiteral { content, .. } => content.to_str().len() as u32,
        }
    }
}
