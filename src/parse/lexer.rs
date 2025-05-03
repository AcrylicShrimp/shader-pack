mod token;
mod token_kind;
mod token_number_literal_kind;

pub use token::*;
pub use token_kind::*;
pub use token_number_literal_kind::*;

use super::low_lexer::{low_token_iter, LowToken, LowTokenKind, LowTokenNumberLiteralKind};
use crate::{
    span::{SourceFile, Span},
    symbol::Symbol,
};
use std::iter::from_fn as iter_from_fn;

pub fn token_iter(file: &SourceFile) -> impl Iterator<Item = Token> + '_ {
    let mut iter = unglued_token_iter(file);
    let mut current = iter.next();
    let mut next = iter.next();

    iter_from_fn(move || {
        let mut token = current.take()?;

        while let Some(next_token) = &next {
            match Token::glue(&token, next_token) {
                Some(glued) => {
                    next = iter.next();
                    token = glued;
                }
                None => {
                    break;
                }
            }
        }

        current = next.take();
        next = iter.next();
        Some(token)
    })
}

fn unglued_token_iter(file: &SourceFile) -> impl Iterator<Item = Token> + '_ {
    let mut span_low = file.span().low();
    let mut iter = low_token_iter(file.content());

    iter_from_fn(move || {
        let token = iter.next()?;
        let length = token.len;
        let token = from_low_token(token, span_low, file);

        span_low += length;
        Some(token)
    })
}

fn from_low_token(token: LowToken, span_low: u32, file: &SourceFile) -> Token {
    let kind = match token.kind {
        LowTokenKind::Unknown => {
            let span = Span::new(span_low, span_low + token.len);
            TokenKind::Unknown {
                len: token.len,
                symbol: Symbol::from_str(file.slice(span)),
            }
        }
        LowTokenKind::EndOfFile => TokenKind::EndOfFile,
        LowTokenKind::Whitespace => TokenKind::Whitespace { len: token.len },
        LowTokenKind::Comment => TokenKind::Comment { len: token.len },
        LowTokenKind::OpenParen => TokenKind::OpenParen,
        LowTokenKind::CloseParen => TokenKind::CloseParen,
        LowTokenKind::OpenBrace => TokenKind::OpenBrace,
        LowTokenKind::CloseBrace => TokenKind::CloseBrace,
        LowTokenKind::OpenBracket => TokenKind::OpenBracket,
        LowTokenKind::CloseBracket => TokenKind::CloseBracket,
        LowTokenKind::Dot => TokenKind::Dot,
        LowTokenKind::Comma => TokenKind::Comma,
        LowTokenKind::Colon => TokenKind::Colon,
        LowTokenKind::Semicolon => TokenKind::Semicolon,
        LowTokenKind::Eq => TokenKind::Assign,
        LowTokenKind::Bang => TokenKind::LogNot,
        LowTokenKind::At => TokenKind::At,
        LowTokenKind::Lt => TokenKind::Lt,
        LowTokenKind::Gt => TokenKind::Gt,
        LowTokenKind::Plus => TokenKind::Add,
        LowTokenKind::Minus => TokenKind::Sub,
        LowTokenKind::Star => TokenKind::Mul,
        LowTokenKind::Slash => TokenKind::Div,
        LowTokenKind::Percent => TokenKind::Mod,
        LowTokenKind::Or => TokenKind::BitOr,
        LowTokenKind::And => TokenKind::BitAnd,
        LowTokenKind::Caret => TokenKind::BitXor,
        LowTokenKind::Tilde => TokenKind::BitNot,
        LowTokenKind::Id => {
            let span = Span::new(span_low, span_low + token.len);
            match file.slice(span) {
                content @ ("true" | "false") => TokenKind::BoolLiteral {
                    len: token.len,
                    content: Symbol::from_str(content),
                },
                id => TokenKind::Id {
                    len: token.len,
                    symbol: Symbol::from_str(id),
                },
            }
        }
        LowTokenKind::NumberLiteral { kind, suffix_start } => {
            let kind = match kind {
                LowTokenNumberLiteralKind::IntegerBinary => TokenNumberLiteralKind::IntegerBinary,
                LowTokenNumberLiteralKind::IntegerOctal => TokenNumberLiteralKind::IntegerOctal,
                LowTokenNumberLiteralKind::IntegerHexadecimal => {
                    TokenNumberLiteralKind::IntegerHexadecimal
                }
                LowTokenNumberLiteralKind::IntegerDecimal => TokenNumberLiteralKind::IntegerDecimal,
                LowTokenNumberLiteralKind::Float => TokenNumberLiteralKind::Float,
            };
            let span = Span::new(span_low, span_low + token.len);
            let content = file.slice(span);
            let content_without_suffix = &content[..suffix_start as usize];
            let suffix = &content[suffix_start as usize..];
            TokenKind::NumberLiteral {
                len: token.len,
                kind,
                content: Symbol::from_str(content_without_suffix),
                suffix: if suffix.is_empty() {
                    None
                } else {
                    Some(Symbol::from_str(suffix))
                },
            }
        }
        LowTokenKind::StringLiteral { terminated } => {
            let span = Span::new(span_low, span_low + token.len);
            let content = file.slice(span);
            let unquoted_content = file.slice(Span::new(
                span.low() + 1,
                if terminated {
                    span.high() - 1
                } else {
                    span.high()
                },
            ));
            TokenKind::StringLiteral {
                len: token.len,
                content: Symbol::from_str(content),
                unquoted_content: Symbol::from_str(unquoted_content),
                terminated,
            }
        }
    };
    Token::new(span_low, kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_token_iter_never_ends() {
        let file = SourceFile::new(0, "", "test", None);
        let mut iter = token_iter(&file);

        for _ in 0..1000 {
            assert!(iter.next() == Some(Token::new(0, TokenKind::EndOfFile)));
        }
    }

    fn random_span_low() -> u32 {
        rand::thread_rng().gen_range(0..u32::MAX / 2)
    }

    fn check_tokens(
        mut span_low: u32,
        input: impl AsRef<str>,
        mut expected: impl Iterator<Item = TokenKind>,
    ) {
        let file = SourceFile::new(span_low, input.as_ref(), "test", None);
        let iter = token_iter(&file);

        for token in iter {
            if token.kind == TokenKind::EndOfFile {
                break;
            }

            assert_eq!(&token, &Token::new(span_low, expected.next().unwrap()));
            span_low += token.kind.len();
        }

        assert_eq!(None, expected.next());
    }

    #[test]
    fn test_token_iter_empty() {
        let span_low = random_span_low();
        check_tokens(span_low, "", [].into_iter());
    }

    #[test]
    fn test_token_iter_unknown() {
        let span_low = random_span_low();
        check_tokens(
            span_low,
            "$$$",
            [TokenKind::Unknown {
                len: 3,
                symbol: Symbol::from_str("$$$"),
            }]
            .into_iter(),
        );
    }

    #[test]
    fn test_token_iter_whitespace() {
        let span_low = random_span_low();
        check_tokens(
            span_low,
            "    \r\r\r\r\n\n\n\n\t\t\t\t",
            [TokenKind::Whitespace { len: 16 }].into_iter(),
        );
    }

    #[test]
    fn test_token_iter_string_literal() {
        let span_low = random_span_low();
        check_tokens(
            span_low,
            "\"hello, world!\" \"\\t\\r\\n\"",
            [
                TokenKind::StringLiteral {
                    len: 15,
                    content: Symbol::from_str("\"hello, world!\""),
                    unquoted_content: Symbol::from_str("hello, world!"),
                    terminated: true,
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::StringLiteral {
                    len: 8,
                    content: Symbol::from_str("\"\\t\\r\\n\""),
                    unquoted_content: Symbol::from_str("\\t\\r\\n"),
                    terminated: true,
                },
            ]
            .into_iter(),
        );
    }

    #[test]
    fn test_token_iter() {
        let span_low = random_span_low();
        let input = " \r\n\t# test\n(){}[].,:;@->=+=-=*=/=%=**=<<=>>=|=&=^===!=<><=>=+-*/%**<<>>|&^||&&~!identifier keyword 0b01_01suffix 0o01234_567suffix 0x0123456789_abcdefsuffix 01234_56789suffix 0123456789.0123456789e-0123456789suffix \"hello, world\" \"hello, world";
        check_tokens(
            span_low,
            input,
            [
                TokenKind::Whitespace { len: 4 },
                TokenKind::Comment { len: 6 },
                TokenKind::Whitespace { len: 1 },
                TokenKind::OpenParen,
                TokenKind::CloseParen,
                TokenKind::OpenBrace,
                TokenKind::CloseBrace,
                TokenKind::OpenBracket,
                TokenKind::CloseBracket,
                TokenKind::Dot,
                TokenKind::Comma,
                TokenKind::Colon,
                TokenKind::Semicolon,
                TokenKind::At,
                TokenKind::Arrow,
                TokenKind::Assign,
                TokenKind::AssignAdd,
                TokenKind::AssignSub,
                TokenKind::AssignMul,
                TokenKind::AssignDiv,
                TokenKind::AssignMod,
                TokenKind::AssignPow,
                TokenKind::AssignShl,
                TokenKind::AssignShr,
                TokenKind::AssignBitOr,
                TokenKind::AssignBitAnd,
                TokenKind::AssignBitXor,
                TokenKind::Eq,
                TokenKind::Ne,
                TokenKind::Lt,
                TokenKind::Gt,
                TokenKind::Le,
                TokenKind::Ge,
                TokenKind::Add,
                TokenKind::Sub,
                TokenKind::Mul,
                TokenKind::Div,
                TokenKind::Mod,
                TokenKind::Pow,
                TokenKind::Shl,
                TokenKind::Shr,
                TokenKind::BitOr,
                TokenKind::BitAnd,
                TokenKind::BitXor,
                TokenKind::LogOr,
                TokenKind::LogAnd,
                TokenKind::BitNot,
                TokenKind::LogNot,
                TokenKind::Id {
                    len: 10,
                    symbol: Symbol::from_str("identifier"),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::Id {
                    len: 7,
                    symbol: Symbol::from_str("keyword"),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::NumberLiteral {
                    len: 13,
                    kind: TokenNumberLiteralKind::IntegerBinary,
                    content: Symbol::from_str("0b01_01"),
                    suffix: Some(Symbol::from_str("suffix")),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::NumberLiteral {
                    len: 17,
                    kind: TokenNumberLiteralKind::IntegerOctal,
                    content: Symbol::from_str("0o01234_567"),
                    suffix: Some(Symbol::from_str("suffix")),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::NumberLiteral {
                    len: 25,
                    kind: TokenNumberLiteralKind::IntegerHexadecimal,
                    content: Symbol::from_str("0x0123456789_abcdef"),
                    suffix: Some(Symbol::from_str("suffix")),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::NumberLiteral {
                    len: 17,
                    kind: TokenNumberLiteralKind::IntegerDecimal,
                    content: Symbol::from_str("01234_56789"),
                    suffix: Some(Symbol::from_str("suffix")),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::NumberLiteral {
                    len: 39,
                    kind: TokenNumberLiteralKind::Float,
                    content: Symbol::from_str("0123456789.0123456789e-0123456789"),
                    suffix: Some(Symbol::from_str("suffix")),
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::StringLiteral {
                    len: 14,
                    content: Symbol::from_str("\"hello, world\""),
                    unquoted_content: Symbol::from_str("hello, world"),
                    terminated: true,
                },
                TokenKind::Whitespace { len: 1 },
                TokenKind::StringLiteral {
                    len: 13,
                    content: Symbol::from_str("\"hello, world"),
                    unquoted_content: Symbol::from_str("hello, world"),
                    terminated: false,
                },
            ]
            .into_iter(),
        );
    }
}
