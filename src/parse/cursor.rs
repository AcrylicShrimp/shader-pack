use super::{
    ast::{AstPuncKind, NodeId, NodeIdAllocator},
    lexer::{Token, TokenKind},
};
use crate::{diagnostics::ItemSender, symbol::Symbol};

pub struct Cursor<'a, T>
where
    T: Iterator<Item = Token>,
{
    lookahead_0: LookaheadToken,
    lookahead_1: LookaheadToken,
    token_stream: T,
    unglue_tokens: bool,
    id_allocator: &'a mut NodeIdAllocator,
    diagnostics_sender: &'a ItemSender,
}

impl<'a, T> Cursor<'a, T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(
        mut token_stream: T,
        id_allocator: &'a mut NodeIdAllocator,
        diagnostics_sender: &'a ItemSender,
    ) -> Self {
        // SAFETY: since the token stream is never empty, we can unwrap
        let lookahead_0 = token_stream.next().unwrap();
        let lookahead_1 = token_stream.next().unwrap();

        Self {
            lookahead_0: LookaheadToken { token: lookahead_0 },
            lookahead_1: LookaheadToken { token: lookahead_1 },
            token_stream,
            unglue_tokens: false,
            id_allocator,
            diagnostics_sender,
        }
    }

    pub fn has_token(&self) -> bool {
        self.lookahead_0.exists()
    }

    pub fn lookahead_0(&self) -> LookaheadToken {
        self.lookahead_0
    }

    pub fn lookahead_1(&self) -> LookaheadToken {
        self.lookahead_1
    }

    pub fn consume(&mut self) {
        // SAFETY: since the token stream is never empty, we can unwrap
        self.lookahead_0 = self.lookahead_1;
        self.lookahead_1 = LookaheadToken {
            token: self.token_stream.next().unwrap(),
        };
    }

    pub fn node_id(&mut self) -> NodeId {
        self.id_allocator.allocate()
    }

    pub fn reporter(&self) -> &ItemSender {
        self.diagnostics_sender
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LookaheadToken {
    pub token: Token,
}

impl LookaheadToken {
    pub fn exists(self) -> bool {
        self.token.kind != TokenKind::EndOfFile
    }

    pub fn is_id(self) -> bool {
        matches!(self.token.kind, TokenKind::Id { .. })
    }

    pub fn is_keyword(self, symbol: Symbol) -> bool {
        matches!(self.token.kind, TokenKind::Id { symbol: id_symbol, .. } if id_symbol == symbol)
    }

    pub fn is_punc(self, kind: AstPuncKind) -> bool {
        kind.matches_token_kind(self.token.kind)
    }

    pub fn is_string_literal(self) -> bool {
        matches!(self.token.kind, TokenKind::StringLiteral { .. })
    }
}
