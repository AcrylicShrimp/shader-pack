use super::{
    ast::{
        AstComposedIdentifier, AstComposedIdentifierArg, AstExpr, AstIdentifier, AstIdentifierKind,
        AstKeyword, AstPunc, AstPuncKind, AstStringLiteral,
    },
    cursor::Cursor,
    lexer::Token,
    symbols::SYMBOL_IDENT,
};
use crate::{parse::lexer::TokenKind, span::Span, symbol::Symbol};

pub trait Parse<T>
where
    Self: Sized,
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self>;
}

impl<T> Parse<T> for AstKeyword
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        match cursor.lookahead_0() {
            Some(Token {
                kind: TokenKind::Id { symbol },
                span_low,
            }) => {
                cursor.consume();
                Some(Self { span_low, symbol })
            }
            _ => None,
        }
    }
}

impl<T> Parse<T> for AstIdentifier
where
    T: Iterator<Item = Token>,
{
    fn parse(cursor: &mut Cursor<T>) -> Option<Self> {
        match cursor.lookahead_0() {
            Some(Token {
                kind: TokenKind::Id { .. },
                ..
            }) => parse_symbol_identifier(cursor),
            Some(Token {
                kind: TokenKind::LogNot { .. },
                ..
            }) => parse_composed_identifier(cursor),
            _ => None,
        }
    }
}

fn parse_kind<T>(cursor: &mut Cursor<T>, kind: TokenKind) -> Option<Token>
where
    T: Iterator<Item = Token>,
{
    match cursor.lookahead_0() {
        Some(token) if token.kind == kind => {
            cursor.consume();
            Some(token)
        }
        _ => None,
    }
}

fn lookahead_punc_0<T>(cursor: &mut Cursor<T>, kind: AstPuncKind) -> bool
where
    T: Iterator<Item = Token>,
{
    match cursor.lookahead_0() {
        Some(Token {
            kind: token_kind, ..
        }) if token_kind == kind.into_token_kind() => true,
        _ => false,
    }
}

fn lookahead_punc_1<T>(cursor: &mut Cursor<T>, kind: AstPuncKind) -> bool
where
    T: Iterator<Item = Token>,
{
    match cursor.lookahead_1() {
        Some(Token {
            kind: token_kind, ..
        }) if token_kind == kind.into_token_kind() => true,
        _ => false,
    }
}

fn parse_punc<T>(cursor: &mut Cursor<T>, kind: AstPuncKind) -> Option<AstPunc>
where
    T: Iterator<Item = Token>,
{
    let punc = parse_kind(cursor, kind.into_token_kind())?;

    Some(AstPunc {
        span_low: punc.span_low,
        kind,
    })
}

fn parse_id<T>(cursor: &mut Cursor<T>) -> Option<(Span, Symbol)>
where
    T: Iterator<Item = Token>,
{
    let id = cursor.lookahead_0()?;

    match id.kind {
        TokenKind::Id { symbol } => {
            cursor.consume();
            Some((id.span(), symbol))
        }
        _ => None,
    }
}

fn parse_keyword<T>(cursor: &mut Cursor<T>, symbol: Symbol) -> Option<AstKeyword>
where
    T: Iterator<Item = Token>,
{
    let (span, keyword_symbol) = parse_id(cursor)?;

    if keyword_symbol != symbol {
        return None;
    }

    Some(AstKeyword {
        span_low: span.low(),
        symbol,
    })
}

fn parse_string_literal<T>(cursor: &mut Cursor<T>) -> Option<AstStringLiteral>
where
    T: Iterator<Item = Token>,
{
    let node_id = cursor.node_id();
    let literal = cursor.lookahead_0()?;

    match literal.kind {
        TokenKind::StringLiteral {
            content,
            unquoted_content,
            terminated,
        } => {
            cursor.consume();
            Some(AstStringLiteral {
                node_id,
                span: literal.span(),
                content,
                unquoted_content,
                terminated,
            })
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

        while cursor.is_exists() && !lookahead_punc_0(cursor, AstPuncKind::CloseParen) {
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
