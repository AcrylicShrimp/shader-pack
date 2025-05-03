mod cursor;
mod low_token;
mod low_token_kind;
mod low_token_number_literal_kind;

pub use low_token::*;
pub use low_token_kind::*;
pub use low_token_number_literal_kind::*;

use self::cursor::Cursor;
use std::iter::from_fn as iter_from_fn;
use unicode_xid::UnicodeXID;

pub fn low_token_iter(mut input: &str) -> impl Iterator<Item = LowToken> + '_ {
    iter_from_fn(move || {
        let token = next(input);
        input = &input[token.len as usize..];
        Some(token)
    })
}

fn next(input: impl AsRef<str>) -> LowToken {
    let mut cursor = Cursor::new(input.as_ref());
    let char = match cursor.consume() {
        Some(char) => char,
        None => return LowToken::new(LowTokenKind::EndOfFile, 0),
    };

    let kind = match char {
        char if char.is_whitespace() => {
            consume_while(&mut cursor, |char| char.is_whitespace());
            LowTokenKind::Whitespace
        }
        char if is_id_start(char) => {
            consume_while(&mut cursor, is_id_continue);
            LowTokenKind::Id
        }
        char @ '0'..='9' => {
            let kind = consume_literal_number(&mut cursor, char);
            let suffix_start = cursor.len_consumed();

            if is_id_start(cursor.first()) {
                cursor.consume();
                consume_while(&mut cursor, is_id_continue);
            }

            LowTokenKind::NumberLiteral { kind, suffix_start }
        }
        '#' => {
            consume_comment(&mut cursor);
            LowTokenKind::Comment
        }
        '(' => LowTokenKind::OpenParen,
        ')' => LowTokenKind::CloseParen,
        '{' => LowTokenKind::OpenBrace,
        '}' => LowTokenKind::CloseBrace,
        '[' => LowTokenKind::OpenBracket,
        ']' => LowTokenKind::CloseBracket,
        '.' => LowTokenKind::Dot,
        ',' => LowTokenKind::Comma,
        ':' => LowTokenKind::Colon,
        ';' => LowTokenKind::Semicolon,
        '=' => LowTokenKind::Eq,
        '!' => LowTokenKind::Bang,
        '@' => LowTokenKind::At,
        '<' => LowTokenKind::Lt,
        '>' => LowTokenKind::Gt,
        '+' => LowTokenKind::Plus,
        '-' => LowTokenKind::Minus,
        '*' => LowTokenKind::Star,
        '/' => LowTokenKind::Slash,
        '%' => LowTokenKind::Percent,
        '|' => LowTokenKind::Or,
        '&' => LowTokenKind::And,
        '^' => LowTokenKind::Caret,
        '~' => LowTokenKind::Tilde,
        '"' => LowTokenKind::StringLiteral {
            terminated: consume_literal_string(&mut cursor),
        },
        _ => LowTokenKind::Unknown,
    };
    LowToken::new(kind, cursor.len_consumed())
}

fn consume_while(cursor: &mut Cursor, mut pred: impl FnMut(char) -> bool) {
    while cursor.is_exists() && pred(cursor.first()) {
        cursor.consume();
    }
}

fn consume_comment(cursor: &mut Cursor) {
    while cursor.is_exists()
        && cursor.first() != '\n'
        && !(cursor.first() == '\r' && cursor.second() == '\n')
    {
        cursor.consume();
    }
}

fn is_id_start(char: char) -> bool {
    char.is_ascii_lowercase()
        || char.is_ascii_uppercase()
        || (char == '_')
        || (char > '\x7f' && char.is_xid_start())
}

fn is_id_continue(char: char) -> bool {
    char.is_ascii_lowercase()
        || char.is_ascii_uppercase()
        || char.is_ascii_digit()
        || char == '_'
        || (char > '\x7f' && char.is_xid_continue())
}

fn consume_literal_number(cursor: &mut Cursor, first_char: char) -> LowTokenNumberLiteralKind {
    let kind = if first_char == '0' {
        match cursor.first() {
            'b' | 'B' if cursor.second().is_digit(2) => {
                cursor.consume();
                consume_while(cursor, |char| char.is_digit(2) || char == '_');
                LowTokenNumberLiteralKind::IntegerBinary
            }
            'o' | 'O' if cursor.second().is_digit(8) => {
                cursor.consume();
                consume_while(cursor, |char| char.is_digit(8) || char == '_');
                LowTokenNumberLiteralKind::IntegerOctal
            }
            'x' | 'X' if cursor.second().is_ascii_hexdigit() => {
                cursor.consume();
                consume_while(cursor, |char| char.is_ascii_hexdigit() || char == '_');
                LowTokenNumberLiteralKind::IntegerHexadecimal
            }
            '0'..='9' => {
                cursor.consume();
                consume_while(cursor, |char| char.is_ascii_digit() || char == '_');
                LowTokenNumberLiteralKind::IntegerDecimal
            }
            '.' | 'e' | 'E' => LowTokenNumberLiteralKind::IntegerDecimal,
            _ => return LowTokenNumberLiteralKind::IntegerDecimal,
        }
    } else {
        LowTokenNumberLiteralKind::IntegerDecimal
    };

    if kind != LowTokenNumberLiteralKind::IntegerDecimal {
        return kind;
    }

    match cursor.first() {
        '.' if cursor.second().is_ascii_digit() => {
            cursor.consume();
            consume_while(cursor, |char| char.is_ascii_digit() || char == '_');

            match (cursor.first(), cursor.second(), cursor.lookup(2)) {
                ('e' | 'E', '+' | '-', digit) if digit.is_ascii_digit() => {
                    cursor.consume();
                    cursor.consume();
                    consume_while(cursor, |char| char.is_ascii_digit() || char == '_');
                }
                ('e' | 'E', digit, _) if digit.is_ascii_digit() => {
                    cursor.consume();
                    consume_while(cursor, |char| char.is_ascii_digit() || char == '_');
                }
                _ => {}
            }

            LowTokenNumberLiteralKind::Float
        }
        'e' | 'E'
            if match cursor.second() {
                '+' | '-' if cursor.lookup(2).is_ascii_digit() => true,
                digit if digit.is_ascii_digit() => true,
                _ => false,
            } =>
        {
            cursor.consume();

            match cursor.first() {
                '+' | '-' => {
                    cursor.consume();
                }
                _ => {}
            }

            consume_while(cursor, |char| char.is_ascii_digit() || char == '_');
            LowTokenNumberLiteralKind::Float
        }
        _ => {
            consume_while(cursor, |char| char.is_ascii_digit() || char == '_');
            LowTokenNumberLiteralKind::IntegerDecimal
        }
    }
}

fn consume_literal_string(cursor: &mut Cursor) -> bool {
    while let Some(char) = cursor.consume() {
        match char {
            '"' => return true,
            '\\' => {
                cursor.consume();
            }
            _ => {}
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_token_next_eof() {
        assert_eq!(next(""), LowToken::new(LowTokenKind::EndOfFile, 0));
    }

    #[test]
    fn test_low_token_next_whitespace() {
        assert_eq!(next(" "), LowToken::new(LowTokenKind::Whitespace, 1));
        assert_eq!(next("\n"), LowToken::new(LowTokenKind::Whitespace, 1));
        assert_eq!(next("\r"), LowToken::new(LowTokenKind::Whitespace, 1));
        assert_eq!(next("\t"), LowToken::new(LowTokenKind::Whitespace, 1));
        assert_eq!(
            next("  \n  \r  \t  \n  "),
            LowToken::new(LowTokenKind::Whitespace, 14)
        );
    }

    #[test]
    fn test_low_token_next_comment() {
        assert_eq!(
            next("# hello, world!"),
            LowToken::new(LowTokenKind::Comment, 15)
        );
        assert_eq!(
            next("# hello, world!\n"),
            LowToken::new(LowTokenKind::Comment, 15)
        );
        assert_eq!(
            next("# hello, world!\n\n"),
            LowToken::new(LowTokenKind::Comment, 15)
        );
        assert_eq!(
            next("# hello, world!\r\n"),
            LowToken::new(LowTokenKind::Comment, 15)
        );
    }

    #[test]
    fn test_low_token_next_punc() {
        assert_eq!(next("("), LowToken::new(LowTokenKind::OpenParen, 1));
        assert_eq!(next(")"), LowToken::new(LowTokenKind::CloseParen, 1));
        assert_eq!(next("{"), LowToken::new(LowTokenKind::OpenBrace, 1));
        assert_eq!(next("}"), LowToken::new(LowTokenKind::CloseBrace, 1));
        assert_eq!(next("["), LowToken::new(LowTokenKind::OpenBracket, 1));
        assert_eq!(next("]"), LowToken::new(LowTokenKind::CloseBracket, 1));
        assert_eq!(next("."), LowToken::new(LowTokenKind::Dot, 1));
        assert_eq!(next(","), LowToken::new(LowTokenKind::Comma, 1));
        assert_eq!(next(":"), LowToken::new(LowTokenKind::Colon, 1));
        assert_eq!(next(";"), LowToken::new(LowTokenKind::Semicolon, 1));
        assert_eq!(next("="), LowToken::new(LowTokenKind::Eq, 1));
        assert_eq!(next("!"), LowToken::new(LowTokenKind::Bang, 1));
        assert_eq!(next("@"), LowToken::new(LowTokenKind::At, 1));
        assert_eq!(next("<"), LowToken::new(LowTokenKind::Lt, 1));
        assert_eq!(next(">"), LowToken::new(LowTokenKind::Gt, 1));
        assert_eq!(next("+"), LowToken::new(LowTokenKind::Plus, 1));
        assert_eq!(next("-"), LowToken::new(LowTokenKind::Minus, 1));
        assert_eq!(next("*"), LowToken::new(LowTokenKind::Star, 1));
        assert_eq!(next("/"), LowToken::new(LowTokenKind::Slash, 1));
        assert_eq!(next("%"), LowToken::new(LowTokenKind::Percent, 1));
        assert_eq!(next("|"), LowToken::new(LowTokenKind::Or, 1));
        assert_eq!(next("&"), LowToken::new(LowTokenKind::And, 1));
        assert_eq!(next("^"), LowToken::new(LowTokenKind::Caret, 1));
        assert_eq!(next("~"), LowToken::new(LowTokenKind::Tilde, 1));
    }

    #[test]
    fn test_low_token_next_id() {
        assert_eq!(next("a"), LowToken::new(LowTokenKind::Id, 1));
        assert_eq!(next("foo"), LowToken::new(LowTokenKind::Id, 3));
        assert_eq!(next("hello-world"), LowToken::new(LowTokenKind::Id, 5));
        assert_eq!(next("_"), LowToken::new(LowTokenKind::Id, 1));
        assert_eq!(next("_foo"), LowToken::new(LowTokenKind::Id, 4));
        assert_eq!(next("foo_"), LowToken::new(LowTokenKind::Id, 4));
        assert_eq!(next("_foo_"), LowToken::new(LowTokenKind::Id, 5));
        assert_eq!(next("test123"), LowToken::new(LowTokenKind::Id, 7));
        assert_eq!(next("test123_123"), LowToken::new(LowTokenKind::Id, 11));
        assert_eq!(next("_123test"), LowToken::new(LowTokenKind::Id, 8));
    }

    #[test]
    fn test_low_token_next_number_literal_integers() {
        assert_eq!(
            next("0b010101"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 8
                },
                8
            )
        );
        assert_eq!(
            next("0B010101"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 8
                },
                8
            )
        );
        assert_eq!(
            next("0b01_01_01_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 11
                },
                11
            )
        );
        assert_eq!(
            next("0B01_01_01_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 11
                },
                11
            )
        );
        assert_eq!(
            next("0b010101suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 8
                },
                8 + 6
            )
        );
        assert_eq!(
            next("0B010101suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 8
                },
                8 + 6
            )
        );
        assert_eq!(
            next("0b01_01_01_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 11
                },
                11 + 6
            )
        );
        assert_eq!(
            next("0B01_01_01_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerBinary,
                    suffix_start: 11
                },
                11 + 6
            )
        );
        assert_eq!(
            next("0o01234567"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 10
                },
                10
            )
        );
        assert_eq!(
            next("0O01234567"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 10
                },
                10
            )
        );
        assert_eq!(
            next("0o01_23_45_67_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 14
                },
                14
            )
        );
        assert_eq!(
            next("0O01_23_45_67_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 14
                },
                14
            )
        );
        assert_eq!(
            next("0o01234567suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 10
                },
                10 + 6
            )
        );
        assert_eq!(
            next("0O01234567suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 10
                },
                10 + 6
            )
        );
        assert_eq!(
            next("0o01_23_45_67_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 14
                },
                14 + 6
            )
        );
        assert_eq!(
            next("0O01_23_45_67_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerOctal,
                    suffix_start: 14
                },
                14 + 6
            )
        );
        assert_eq!(
            next("0x0123456789abcdef"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 18
                },
                18
            )
        );
        assert_eq!(
            next("0X0123456789ABCDEF"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 18
                },
                18
            )
        );
        assert_eq!(
            next("0x01_23_45_67_89_ab_cd_ef_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 26
                },
                26
            )
        );
        assert_eq!(
            next("0X01_23_45_67_89_AB_CD_EF_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 26
                },
                26
            )
        );
        assert_eq!(
            next("0x0123456789abcdefsuffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 18
                },
                18 + 6
            )
        );
        assert_eq!(
            next("0X0123456789ABCDEFsuffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 18
                },
                18 + 6
            )
        );
        assert_eq!(
            next("0x01_23_45_67_89_ab_cd_ef_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 26
                },
                26 + 6
            )
        );
        assert_eq!(
            next("0X01_23_45_67_89_AB_CD_EF_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerHexadecimal,
                    suffix_start: 26
                },
                26 + 6
            )
        );
        assert_eq!(
            next("0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerDecimal,
                    suffix_start: 10
                },
                10
            )
        );
        assert_eq!(
            next("01_23_45_67_89_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerDecimal,
                    suffix_start: 15
                },
                15
            )
        );
        assert_eq!(
            next("0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerDecimal,
                    suffix_start: 10
                },
                10 + 6
            )
        );
        assert_eq!(
            next("01_23_45_67_89_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::IntegerDecimal,
                    suffix_start: 15
                },
                15 + 6
            )
        );
    }

    #[test]
    fn test_low_token_next_number_literal_floats() {
        assert_eq!(
            next("0123456789e0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 21
                },
                21
            )
        );
        assert_eq!(
            next("0123456789E0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 21
                },
                21
            )
        );
        assert_eq!(
            next("01_23_45_67_89_e01_23_45_67_89_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 31
                },
                31
            )
        );
        assert_eq!(
            next("01_23_45_67_89_E01_23_45_67_89_"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 31
                },
                31
            )
        );
        assert_eq!(
            next("0123456789e+0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22
            )
        );
        assert_eq!(
            next("0123456789E+0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22
            )
        );
        assert_eq!(
            next("0123456789e-0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22
            )
        );
        assert_eq!(
            next("0123456789E-0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22
            )
        );
        assert_eq!(
            next("0123456789.0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 21
                },
                21
            )
        );
        assert_eq!(
            next("0123456789.0123456789e0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 32
                },
                32
            )
        );
        assert_eq!(
            next("0123456789.0123456789E0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 32
                },
                32
            )
        );
        assert_eq!(
            next("0123456789.0123456789e+0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33
            )
        );
        assert_eq!(
            next("0123456789.0123456789E+0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33
            )
        );
        assert_eq!(
            next("0123456789.0123456789e-0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33
            )
        );
        assert_eq!(
            next("0123456789.0123456789E-0123456789"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33
            )
        );
        assert_eq!(
            next("0123456789e0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 21
                },
                21 + 6
            )
        );
        assert_eq!(
            next("0123456789E0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 21
                },
                21 + 6
            )
        );
        assert_eq!(
            next("01_23_45_67_89_e01_23_45_67_89_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 31
                },
                31 + 6
            )
        );
        assert_eq!(
            next("01_23_45_67_89_E01_23_45_67_89_suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 31
                },
                31 + 6
            )
        );
        assert_eq!(
            next("0123456789e+0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22 + 6
            )
        );
        assert_eq!(
            next("0123456789E+0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22 + 6
            )
        );
        assert_eq!(
            next("0123456789e-0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22 + 6
            )
        );
        assert_eq!(
            next("0123456789E-0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 22
                },
                22 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 21
                },
                21 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789e0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 32
                },
                32 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789E0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 32
                },
                32 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789e+0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789E+0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789e-0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33 + 6
            )
        );
        assert_eq!(
            next("0123456789.0123456789E-0123456789suffix"),
            LowToken::new(
                LowTokenKind::NumberLiteral {
                    kind: LowTokenNumberLiteralKind::Float,
                    suffix_start: 33
                },
                33 + 6
            )
        );
    }

    #[test]
    fn test_low_token_next_string_literal() {
        assert_eq!(
            next("\"hello\""),
            LowToken::new(LowTokenKind::StringLiteral { terminated: true }, 7)
        );
        assert_eq!(
            next("\"hello\n\""),
            LowToken::new(LowTokenKind::StringLiteral { terminated: true }, 8)
        );

        assert_eq!(
            next("\"hello"),
            LowToken::new(LowTokenKind::StringLiteral { terminated: false }, 6)
        );
        assert_eq!(
            next("\"hello\n"),
            LowToken::new(LowTokenKind::StringLiteral { terminated: false }, 7)
        );
    }

    fn check_low_tokens<'a>(
        input: impl AsRef<str>,
        mut expected: impl Iterator<Item = &'a LowToken>,
    ) {
        let iter = low_token_iter(input.as_ref());

        for token in iter {
            if token.kind == LowTokenKind::EndOfFile {
                break;
            }

            assert_eq!(&token, expected.next().unwrap());
        }

        assert_eq!(None, expected.next());
    }

    #[test]
    fn test_low_token_iter() {
        check_low_tokens("", [].iter());
        check_low_tokens(
            " \r\n\t# test\n(){}[].,:;=!@<>+-*/%|&^~identifier keyword 0b01_01suffix 0o01234_567suffix 0x0123456789_abcdefsuffix 01234_56789suffix 0123456789.0123456789e-0123456789suffix \"hello, world\" \"hello, world",
            [
                LowToken::new(LowTokenKind::Whitespace, 4),
                LowToken::new(LowTokenKind::Comment, 6),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::OpenParen, 1),
                LowToken::new(LowTokenKind::CloseParen, 1),
                LowToken::new(LowTokenKind::OpenBrace, 1),
                LowToken::new(LowTokenKind::CloseBrace, 1),
                LowToken::new(LowTokenKind::OpenBracket, 1),
                LowToken::new(LowTokenKind::CloseBracket, 1),
                LowToken::new(LowTokenKind::Dot, 1),
                LowToken::new(LowTokenKind::Comma, 1),
                LowToken::new(LowTokenKind::Colon, 1),
                LowToken::new(LowTokenKind::Semicolon, 1),
                LowToken::new(LowTokenKind::Eq, 1),
                LowToken::new(LowTokenKind::Bang, 1),
                LowToken::new(LowTokenKind::At, 1),
                LowToken::new(LowTokenKind::Lt, 1),
                LowToken::new(LowTokenKind::Gt, 1),
                LowToken::new(LowTokenKind::Plus, 1),
                LowToken::new(LowTokenKind::Minus, 1),
                LowToken::new(LowTokenKind::Star, 1),
                LowToken::new(LowTokenKind::Slash, 1),
                LowToken::new(LowTokenKind::Percent, 1),
                LowToken::new(LowTokenKind::Or, 1),
                LowToken::new(LowTokenKind::And, 1),
                LowToken::new(LowTokenKind::Caret, 1),
                LowToken::new(LowTokenKind::Tilde, 1),
                LowToken::new(LowTokenKind::Id, 10),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::Id, 7),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::NumberLiteral { kind: LowTokenNumberLiteralKind::IntegerBinary, suffix_start: 7 }, 7 + 6),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::NumberLiteral { kind: LowTokenNumberLiteralKind::IntegerOctal, suffix_start: 11 }, 11 + 6),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::NumberLiteral { kind: LowTokenNumberLiteralKind::IntegerHexadecimal, suffix_start: 19 }, 19 + 6),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::NumberLiteral { kind: LowTokenNumberLiteralKind::IntegerDecimal, suffix_start: 11 }, 11 + 6),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::NumberLiteral { kind: LowTokenNumberLiteralKind::Float, suffix_start: 33 }, 33 + 6),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::StringLiteral { terminated: true }, 14),
                LowToken::new(LowTokenKind::Whitespace, 1),
                LowToken::new(LowTokenKind::StringLiteral { terminated: false }, 13),

            ]
            .iter(),
        );
    }
}
