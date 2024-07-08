use super::TokenNumberLiteralKind;
use crate::{span::Span, symbol::Symbol};

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
    pub fn span(self, span_low: u32) -> Span {
        Span::new(span_low, span_low + self.len())
    }

    pub fn len(self) -> u32 {
        match self {
            TokenKind::Unknown { symbol } => symbol.to_str().len() as u32,
            TokenKind::Whitespace { len } => len,
            TokenKind::Comment { len } => len,
            TokenKind::OpenParen => 1,
            TokenKind::CloseParen => 1,
            TokenKind::OpenBrace => 1,
            TokenKind::CloseBrace => 1,
            TokenKind::OpenBracket => 1,
            TokenKind::CloseBracket => 1,
            TokenKind::Dot => 1,
            TokenKind::Comma => 1,
            TokenKind::Colon => 1,
            TokenKind::Semicolon => 1,
            TokenKind::At => 1,
            TokenKind::Arrow => 2,
            TokenKind::Assign => 1,
            TokenKind::AssignAdd => 2,
            TokenKind::AssignSub => 2,
            TokenKind::AssignMul => 2,
            TokenKind::AssignDiv => 2,
            TokenKind::AssignMod => 2,
            TokenKind::AssignPow => 3,
            TokenKind::AssignShl => 3,
            TokenKind::AssignShr => 3,
            TokenKind::AssignBitOr => 2,
            TokenKind::AssignBitAnd => 2,
            TokenKind::AssignBitXor => 2,
            TokenKind::Eq => 2,
            TokenKind::Ne => 2,
            TokenKind::Lt => 1,
            TokenKind::Gt => 1,
            TokenKind::Le => 2,
            TokenKind::Ge => 2,
            TokenKind::Add => 1,
            TokenKind::Sub => 1,
            TokenKind::Mul => 1,
            TokenKind::Div => 1,
            TokenKind::Mod => 1,
            TokenKind::Pow => 2,
            TokenKind::Shl => 2,
            TokenKind::Shr => 2,
            TokenKind::BitOr => 1,
            TokenKind::BitAnd => 1,
            TokenKind::BitXor => 1,
            TokenKind::LogOr => 2,
            TokenKind::LogAnd => 2,
            TokenKind::BitNot => 1,
            TokenKind::LogNot => 1,
            TokenKind::Id { symbol } => symbol.to_str().len() as u32,
            TokenKind::BoolLiteral { content } => content.to_str().len() as u32,
            TokenKind::NumberLiteral {
                content, suffix, ..
            } => {
                let content_len = content.to_str().len();
                let suffix_len = suffix.map_or(0, |s| s.to_str().len());
                (content_len + suffix_len) as u32
            }
            TokenKind::StringLiteral { content, .. } => content.to_str().len() as u32,
        }
    }
}
