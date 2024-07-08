use super::TokenKind;
use crate::{span::Span, symbol::Symbol};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    pub span_low: u32,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(span_low: u32, kind: TokenKind) -> Self {
        Self { span_low, kind }
    }

    pub fn span(self) -> Span {
        self.kind.span(self.span_low)
    }

    pub fn len(self) -> u32 {
        self.kind.len()
    }

    pub fn glue(lhs: &Self, rhs: &Self) -> Option<Self> {
        let kind = match (lhs.kind, rhs.kind) {
            (
                TokenKind::Unknown { symbol: lhs_symbol },
                TokenKind::Unknown { symbol: rhs_symbol },
            ) => {
                let lhs = lhs_symbol.to_str();
                let rhs = rhs_symbol.to_str();
                let symbol = format!("{}{}", lhs, rhs);
                TokenKind::Unknown {
                    symbol: Symbol::from_str(symbol),
                }
            }
            (TokenKind::Whitespace { len: lhs_len }, TokenKind::Whitespace { len: rhs_len }) => {
                TokenKind::Whitespace {
                    len: lhs_len + rhs_len,
                }
            }
            (TokenKind::Assign, TokenKind::Assign) => TokenKind::Eq, // `==`
            (TokenKind::Lt, TokenKind::Assign) => TokenKind::Le,     // `<=`
            (TokenKind::Lt, TokenKind::Lt) => TokenKind::Shl,        // `<<`
            (TokenKind::Gt, TokenKind::Assign) => TokenKind::Ge,     // `>=`
            (TokenKind::Gt, TokenKind::Gt) => TokenKind::Shr,        // `>>`
            (TokenKind::Add, TokenKind::Assign) => TokenKind::AssignAdd, // `+=`
            (TokenKind::Sub, TokenKind::Assign) => TokenKind::AssignSub, // `-=`
            (TokenKind::Sub, TokenKind::Gt) => TokenKind::Arrow,     // `->`
            (TokenKind::Mul, TokenKind::Assign) => TokenKind::AssignMul, // `*=`
            (TokenKind::Mul, TokenKind::Mul) => TokenKind::Pow,      // `**`
            (TokenKind::Div, TokenKind::Assign) => TokenKind::AssignDiv, // `/=`
            (TokenKind::Mod, TokenKind::Assign) => TokenKind::AssignMod, // `%=`
            (TokenKind::Pow, TokenKind::Assign) => TokenKind::AssignPow, // `**=`
            (TokenKind::Shl, TokenKind::Assign) => TokenKind::AssignShl, // `<<=`
            (TokenKind::Shr, TokenKind::Assign) => TokenKind::AssignShr, // `>>=`
            (TokenKind::BitOr, TokenKind::Assign) => TokenKind::AssignBitOr, // `|=`
            (TokenKind::BitOr, TokenKind::BitOr) => TokenKind::LogOr, // `||`
            (TokenKind::BitAnd, TokenKind::Assign) => TokenKind::AssignBitAnd, // `&=`
            (TokenKind::BitAnd, TokenKind::BitAnd) => TokenKind::LogAnd, // `&&`
            (TokenKind::BitXor, TokenKind::Assign) => TokenKind::AssignBitXor, // `^=`
            (TokenKind::LogNot, TokenKind::Assign) => TokenKind::Ne, // `!=`
            _ => {
                return None;
            }
        };
        Some(Self::new(lhs.span_low, kind))
    }

    pub fn unglue(this: Self, unglued: &mut VecDeque<Self>) {
        match this.kind {
            TokenKind::Unknown { .. } | TokenKind::Whitespace { .. } => {
                // `Unknown` and `Whitespace` are cannot be unglued
                unglued.push_back(this);
            }
            TokenKind::Eq => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Assign,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Le => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Lt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Shl => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Lt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Lt,
                });
            }
            TokenKind::Ge => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Gt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Shr => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Gt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Gt,
                });
            }
            TokenKind::AssignAdd => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Add,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignSub => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Sub,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Arrow => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Sub,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Gt,
                });
            }
            TokenKind::AssignMul => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Mul,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Pow => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Mul,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Mul,
                });
            }
            TokenKind::AssignDiv => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Div,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignMod => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Mod,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignPow => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Mul,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Mul,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 2,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignShl => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Lt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Lt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 2,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignShr => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::Gt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Gt,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 2,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::AssignBitOr => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::BitOr,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::LogOr => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::BitOr,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::BitOr,
                });
            }
            TokenKind::AssignBitAnd => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::BitAnd,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::LogAnd => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::BitAnd,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::BitAnd,
                });
            }
            TokenKind::AssignBitXor => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::BitXor,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            TokenKind::Ne => {
                unglued.push_back(Self {
                    span_low: this.span_low + 0,
                    kind: TokenKind::LogNot,
                });
                unglued.push_back(Self {
                    span_low: this.span_low + 1,
                    kind: TokenKind::Assign,
                });
            }
            _ => {
                unglued.push_back(this);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_span_low() -> u32 {
        rand::thread_rng().gen_range(0..u32::MAX / 2)
    }

    fn make_token(kind: TokenKind) -> Token {
        Token::new(random_span_low(), kind)
    }

    fn check_unglue_glue(token: Token) {
        println!("unglueing: {:?}", token);

        let mut buf = VecDeque::with_capacity(3);
        Token::unglue(token.clone(), &mut buf);

        for token in &buf {
            println!("- unglued: {:?}", token);
            assert_eq!(token.len(), 1);
        }

        let mut glued = buf.pop_front().unwrap();

        while let Some(rhs) = buf.pop_front() {
            glued = Token::glue(&glued, &rhs).unwrap();
        }

        assert_eq!(glued, token);
    }

    #[test]
    fn test_token_unglue_glue() {
        let kinds = [
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
        ];

        for kind in kinds {
            check_unglue_glue(make_token(kind));
        }
    }
}
