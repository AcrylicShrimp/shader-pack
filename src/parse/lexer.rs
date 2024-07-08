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
                symbol: Symbol::from_str(file.slice(span)),
            }
        }
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
        LowTokenKind::Eq => TokenKind::Eq,
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
                    content: Symbol::from_str(content),
                },
                id => TokenKind::Id {
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

    fn random_span_low() -> u32 {
        rand::thread_rng().gen_range(0..u32::MAX / 2)
    }

    fn check_tokens<'a>(
        span_low: u32,
        input: impl AsRef<str>,
        mut expected: impl Iterator<Item = &'a Token>,
    ) {
        let file = SourceFile::new(span_low, input.as_ref(), "test", None);
        let mut iter = token_iter(&file);

        while let Some(token) = iter.next() {
            assert_eq!(&token, expected.next().unwrap());
        }

        assert_eq!(None, expected.next());
    }

    #[test]
    fn test_token_iter_empty() {
        let span_low = random_span_low();
        check_tokens(span_low, "", [].iter());
    }

    #[test]
    fn test_token_iter_unknown() {
        let span_low = random_span_low();
        check_tokens(
            span_low,
            "$$$",
            [Token::new(
                span_low + 0,
                TokenKind::Unknown {
                    symbol: Symbol::from_str("$$$"),
                },
            )]
            .iter(),
        );
    }

    #[test]
    fn test_token_iter_whitespace() {
        let span_low = random_span_low();
        check_tokens(
            span_low,
            "    \r\r\r\r\n\n\n\n\t\t\t\t",
            [Token::new(span_low + 0, TokenKind::Whitespace { len: 16 })].iter(),
        );
    }

    #[test]
    fn test_token_iter() {
        // TODO: implement this test
    }
}
