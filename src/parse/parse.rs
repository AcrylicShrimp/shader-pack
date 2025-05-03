use super::{
    ast::{
        AstAttribute, AstAttributeItem, AstCompTime, AstCompTimeBlock, AstCompTimeElseIfPart,
        AstCompTimeElsePart, AstCompTimeIf, AstCompTimeIfPart, AstCompTimeIfPredicateExpr,
        AstCompTimeIfPredicateExprKind, AstCompTimeIfPredicateExprNot,
        AstCompTimeIfPredicateExprParen, AstCompTimeLoop, AstComposedIdentifier,
        AstComposedIdentifierArg, AstExpr, AstIdentifier, AstIdentifierKind, AstKeyword, AstPunc,
        AstPuncKind, AstShaderPack, AstStringLiteral, AstTopLevel,
    },
    cursor::Cursor,
    lexer::Token,
};
use crate::{
    diagnostics::codes::PARSE_ERR_INVALID_COMPTIME,
    parse::{
        ast::{
            AstCompTimeIfPredicateExprAnd, AstCompTimeIfPredicateExprFlag,
            AstCompTimeIfPredicateExprOr, AstCompTimeKind,
        },
        lexer::TokenKind,
    },
    span::Span,
};

pub trait Parse<T>
where
    Self: Sized,
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self>;
}

impl<T> Parse<T> for AstShaderPack
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        todo!()
    }
}

impl<T> Parse<T> for AstTopLevel
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        todo!()
    }
}

impl<T> Parse<T> for AstAttribute
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let mut items = Vec::new();

        while cursor.lookahead_0().is_punc(AstPuncKind::At) {
            let item = AstAttributeItem::parse(cursor)?;
            items.push(item);
        }

        Some(AstAttribute {
            node_id,
            span: items.first().unwrap().span,
            items,
        })
    }
}

impl<T> Parse<T> for AstAttributeItem
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let punc_at = parse_punc(cursor, AstPuncKind::At)?;
        let ident = AstIdentifier::parse(cursor)?;
        let punc_assign = parse_punc(cursor, AstPuncKind::Assign)?;
        let expr = AstStringLiteral::parse(cursor)?;

        Some(AstAttributeItem {
            node_id,
            span: punc_at.span(),
            punc_at,
            ident,
            punc_assign,
            expr,
        })
    }
}

impl<T, I> Parse<T> for AstCompTime<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let keyword_comptime = parse_keyword(cursor, *KEYWORD_COMPTIME)?;

        if cursor.lookahead_0().is_keyword(*KEYWORD_IF) {
            let comptime_if = AstCompTimeIf::parse(cursor)?;
            let span = keyword_comptime.span.expand_to(comptime_if.span.high());
            let kind = AstCompTimeKind::If(comptime_if);

            return Some(AstCompTime {
                node_id,
                span,
                keyword_comptime,
                kind,
            });
        }

        cursor.reporter().error(
            PARSE_ERR_INVALID_COMPTIME,
            keyword_comptime.span,
            "`comptime` must be followed by `if` or `loop`",
        );

        None
    }
}

impl<T, I> Parse<T> for AstCompTimeIf<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let if_part = AstCompTimeIfPart::parse(cursor)?;

        let mut else_if_parts = Vec::new();

        while cursor.lookahead_0().is_keyword(*KEYWORD_ELSE)
            && cursor.lookahead_1().is_keyword(*KEYWORD_IF)
        {
            let else_if_part = AstCompTimeElseIfPart::parse(cursor)?;
            else_if_parts.push(else_if_part);
        }

        let else_part = if cursor.lookahead_0().is_keyword(*KEYWORD_ELSE) {
            let else_part = AstCompTimeElsePart::parse(cursor)?;
            Some(else_part)
        } else {
            None
        };

        Some(AstCompTimeIf {
            node_id,
            span: if_part.span,
            if_part,
            else_if_parts,
            else_part,
        })
    }
}

impl<T, I> Parse<T> for AstCompTimeIfPart<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let keyword_if = parse_keyword(cursor, *KEYWORD_IF)?;
        let predicate = AstCompTimeIfPredicateExpr::parse(cursor)?;
        let block = AstCompTimeBlock::parse(cursor)?;
        let span = keyword_if.span.expand_to(block.span.high());

        Some(AstCompTimeIfPart {
            node_id,
            span,
            keyword_if,
            predicate,
            block,
        })
    }
}

impl<T, I> Parse<T> for AstCompTimeElseIfPart<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let keyword_else = parse_keyword(cursor, *KEYWORD_ELSE)?;
        let keyword_if = parse_keyword(cursor, *KEYWORD_IF)?;
        let predicate = AstCompTimeIfPredicateExpr::parse(cursor)?;
        let block = AstCompTimeBlock::parse(cursor)?;
        let span = keyword_else.span.expand_to(block.span.high());

        Some(AstCompTimeElseIfPart {
            node_id,
            span,
            keyword_else,
            keyword_if,
            predicate,
            block,
        })
    }
}

impl<T, I> Parse<T> for AstCompTimeElsePart<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let keyword_else = parse_keyword(cursor, *KEYWORD_ELSE)?;
        let block = AstCompTimeBlock::parse(cursor)?;
        let span = keyword_else.span.expand_to(block.span.high());

        Some(AstCompTimeElsePart {
            node_id,
            span,
            keyword_else,
            block,
        })
    }
}

impl<T> Parse<T> for AstCompTimeIfPredicateExpr
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let (mut expr_span, mut expr) = if cursor.lookahead_0().is_string_literal() {
            let node_id = cursor.node_id();
            let expr = AstCompTimeIfPredicateExprFlag::parse(cursor)?;
            let span = expr.span;

            (
                span,
                AstCompTimeIfPredicateExpr {
                    node_id,
                    span,
                    kind: AstCompTimeIfPredicateExprKind::Flag(expr),
                },
            )
        } else if cursor.lookahead_0().is_punc(AstPuncKind::OpenParen) {
            let node_id = cursor.node_id();
            let expr = AstCompTimeIfPredicateExprParen::parse(cursor)?;
            let span = expr.span;

            (
                span,
                AstCompTimeIfPredicateExpr {
                    node_id,
                    span,
                    kind: AstCompTimeIfPredicateExprKind::Paren(expr),
                },
            )
        } else if cursor.lookahead_0().is_keyword(*KEYWORD_NOT) {
            let node_id = cursor.node_id();
            let expr = AstCompTimeIfPredicateExprNot::parse(cursor)?;
            let span = expr.span;

            (
                span,
                AstCompTimeIfPredicateExpr {
                    node_id,
                    span,
                    kind: AstCompTimeIfPredicateExprKind::Not(expr),
                },
            )
        } else {
            cursor.reporter().error(
                PARSE_ERR_INVALID_COMPTIME,
                cursor.lookahead_0().token.unwrap().span(),
                "`if` predicate must be a string literal, a parenthesized expression, or a `not` expression",
            );

            let node_id = cursor.node_id();
            let span = Span::invalid();

            (
                span,
                AstCompTimeIfPredicateExpr {
                    node_id,
                    span,
                    kind: AstCompTimeIfPredicateExprKind::Invalid,
                },
            )
        };

        while cursor.lookahead_0().exists() {
            if cursor.lookahead_0().is_keyword(*KEYWORD_AND) {
                let node_id = cursor.node_id();
                let keyword_and = parse_keyword(cursor, *KEYWORD_AND)?;
                let rhs = AstCompTimeIfPredicateExpr::parse(cursor)?;
                let span = expr_span.expand_to(rhs.span.high());

                expr_span = span;
                expr = AstCompTimeIfPredicateExpr {
                    node_id,
                    span,
                    kind: AstCompTimeIfPredicateExprKind::And(AstCompTimeIfPredicateExprAnd {
                        node_id,
                        span,
                        lhs: Box::new(expr),
                        keyword_and,
                        rhs: Box::new(rhs),
                    }),
                };
            } else if cursor.lookahead_0().is_keyword(*KEYWORD_OR) {
                let node_id = cursor.node_id();
                let keyword_or = parse_keyword(cursor, *KEYWORD_OR)?;
                let rhs = AstCompTimeIfPredicateExpr::parse(cursor)?;
                let span = expr.span.expand_to(rhs.span.high());

                expr_span = span;
                expr = AstCompTimeIfPredicateExpr {
                    node_id,
                    span,
                    kind: AstCompTimeIfPredicateExprKind::Or(AstCompTimeIfPredicateExprOr {
                        node_id,
                        span,
                        lhs: Box::new(expr),
                        keyword_or,
                        rhs: Box::new(rhs),
                    }),
                };
            } else {
                break;
            }
        }

        todo!()
    }
}

impl<T> Parse<T> for AstCompTimeIfPredicateExprFlag
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let flag = AstStringLiteral::parse(cursor)?;

        Some(AstCompTimeIfPredicateExprFlag {
            node_id,
            span: flag.span,
            flag,
        })
    }
}

impl<T> Parse<T> for AstCompTimeIfPredicateExprParen
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let punc_open_paren = parse_punc(cursor, AstPuncKind::OpenParen)?;
        let expr = AstCompTimeIfPredicateExpr::parse(cursor)?;
        let punc_close_paren =
            if let Some(punc_close_paren) = parse_punc(cursor, AstPuncKind::CloseParen) {
                punc_close_paren
            } else {
                cursor.reporter().error(
                    PARSE_ERR_INVALID_COMPTIME,
                    cursor.lookahead_0().token.unwrap().span(),
                    "`)` is expected",
                );

                AstPunc {
                    span_low: u32::MAX,
                    kind: AstPuncKind::CloseParen,
                }
            };

        Some(AstCompTimeIfPredicateExprParen {
            node_id,
            span: punc_open_paren.span(),
            punc_open_paren,
            expr: Box::new(expr),
            punc_close_paren,
        })
    }
}

impl<T> Parse<T> for AstCompTimeIfPredicateExprNot
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let keyword_not = parse_keyword(cursor, *KEYWORD_NOT)?;
        let expr = AstCompTimeIfPredicateExpr::parse(cursor)?;
        let span = keyword_not.span.expand_to(expr.span.high());

        Some(AstCompTimeIfPredicateExprNot {
            node_id,
            span,
            keyword_not,
            expr: Box::new(expr),
        })
    }
}

impl<T, I> Parse<T> for AstCompTimeLoop<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let keyword_loop = parse_keyword(cursor, *KEYWORD_LOOP)?;
        let loop_var_ident = AstIdentifier::parse(cursor)?;
        let keyword_times = parse_keyword(cursor, *KEYWORD_TIMES)?;
        let expr = AstExpr::parse(cursor)?;
        let block = AstCompTimeBlock::parse(cursor)?;
        let span = keyword_loop.span.expand_to(block.span.high());

        Some(AstCompTimeLoop {
            node_id,
            span,
            keyword_loop,
            loop_var_ident,
            keyword_times,
            expr,
            block,
        })
    }
}

impl<T, I> Parse<T> for AstCompTimeBlock<I>
where
    T: Iterator<Item = Token>,
    I: Parse<T>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let node_id = cursor.node_id();
        let punc_open_brace = parse_punc(cursor, AstPuncKind::OpenBrace)?;

        let mut items = Vec::new();

        while cursor.lookahead_0().exists()
            && !cursor.lookahead_0().is_punc(AstPuncKind::CloseBrace)
        {
            let item = I::parse(cursor)?;
            items.push(item);
        }

        let punc_close_brace = parse_punc(cursor, AstPuncKind::CloseBrace)?;
        let span = punc_open_brace
            .span()
            .expand_to(punc_close_brace.span().high());

        Some(AstCompTimeBlock {
            node_id,
            span,
            punc_open_brace,
            items,
            punc_close_brace,
        })
    }
}

impl<T> Parse<T> for AstExpr
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        todo!()
    }
}

impl<T> Parse<T> for AstIdentifier
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        todo!()
    }
}

impl<T> Parse<T> for AstStringLiteral
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        let literal = cursor.lookahead_0();

        match literal.token {
            Some(Token {
                span_low,
                kind:
                    TokenKind::StringLiteral {
                        len,
                        content,
                        unquoted_content,
                        terminated,
                    },
                ..
            }) => {
                let node_id = cursor.node_id();
                cursor.consume();
                Some(AstStringLiteral {
                    node_id,
                    span: Span::new(span_low, span_low + len),
                    content,
                    unquoted_content,
                    terminated,
                })
            }
            _ => None,
        }
    }
}

pub fn parse_keyword<T>(cursor: &mut Cursor<T>, keyword: Keyword) -> Option<AstKeyword>
where
    T: Iterator<Item = Token>,
{
    match cursor.lookahead_0().token {
        Some(Token {
            span_low,
            kind: TokenKind::Id { symbol },
            ..
        }) if symbol == keyword.symbol => {
            cursor.consume();
            Some(AstKeyword {
                span: Span::new(span_low, span_low + keyword.len),
                symbol,
            })
        }
        _ => None,
    }
}

pub fn parse_punc<T>(cursor: &mut Cursor<T>, kind: AstPuncKind) -> AstPunc
where
    T: Iterator<Item = Token>,
{
    match cursor.lookahead_0().token {
        Some(token) if token.kind == kind.into_token_kind() => {
            cursor.consume();
            AstPunc {
                kind,
                span_low: token.span_low,
            }
        }
        _ => None,
    }
}

fn parse_symbol_identifier<T>(cursor: &mut Cursor<T>) -> Option<AstIdentifier>
where
    T: Iterator<Item = Token>,
{
    let node_id = cursor.node_id();
    let (span, symbol) = parse_id(cursor)?;

    Some(AstIdentifier {
        node_id,
        span,
        kind: AstIdentifierKind::Symbol(symbol),
    })
}

fn parse_composed_identifier<T>(cursor: &mut Cursor<T>) -> Option<AstIdentifier>
where
    T: Iterator<Item = Token>,
{
    let node_id = cursor.node_id();
    let punc_bang = parse_punc(cursor, AstPuncKind::LogNot)?;
    let keyword_ident = parse_keyword(cursor, *SYMBOL_IDENT)?;
    let punc_open_paren = parse_punc(cursor, AstPuncKind::OpenParen)?;
    let rule_str = parse_string_literal(cursor)?;
    let (punc_comma, args) = if lookahead_punc_0(cursor, AstPuncKind::CloseParen) {
        (None, Vec::new())
    } else {
        let punc_comma = parse_punc(cursor, AstPuncKind::Comma)?;
        let mut args = Vec::new();

        while cursor.has_token() && !lookahead_punc_0(cursor, AstPuncKind::CloseParen) {
            let node_id = cursor.node_id();
            let expr = parse_expr(cursor)?;
            let punc_comma = parse_punc(cursor, AstPuncKind::Comma);
            let span = match punc_comma {
                Some(punc_comma) => expr.span.expand_to(punc_comma.span().high()),
                None => expr.span,
            };
            let is_punc_comma_exists = punc_comma.is_some();

            args.push(AstComposedIdentifierArg {
                node_id,
                span,
                expr,
                punc_comma,
            });

            if !is_punc_comma_exists {
                break;
            }
        }

        (Some(punc_comma), args)
    };
    let punc_close_paren = parse_punc(cursor, AstPuncKind::CloseParen)?;

    Some(AstIdentifier {
        node_id,
        span: punc_bang.span(),
        kind: AstIdentifierKind::Composed(AstComposedIdentifier {
            punc_bang,
            keyword_ident,
            punc_open_paren,
            rule_str,
            punc_comma,
            args,
            punc_close_paren,
        }),
    })
}

fn parse_expr<T>(cursor: &mut Cursor<T>) -> Option<AstExpr>
where
    T: Iterator<Item = Token>,
{
    let node_id = cursor.node_id();
    todo!()
}
