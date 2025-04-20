use super::{
    ast::{NodeId, NodeIdAllocator},
    lexer::Token,
};
use crate::diagnostics::ItemSender;

pub struct Cursor<'a, T>
where
    T: Iterator<Item = Token>,
{
    lookahead_0: Option<Token>,
    lookahead_1: Option<Token>,
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
        let lookahead_0 = token_stream.next();
        let lookahead_1 = token_stream.next();

        Self {
            lookahead_0,
            lookahead_1,
            token_stream,
            unglue_tokens: false,
            id_allocator,
            diagnostics_sender,
        }
    }

    pub fn pos(&self) -> u32 {
        match self.lookahead_0() {
            Some(token) => token.span_low,
            None => self.diagnostics_sender.file().span().high(),
        }
    }

    pub fn is_exists(&self) -> bool {
        self.lookahead_0.is_some()
    }

    pub fn lookahead_0(&self) -> Option<Token> {
        self.lookahead_0
    }

    pub fn lookahead_1(&self) -> Option<Token> {
        self.lookahead_1
    }

    pub fn consume(&mut self) {
        self.lookahead_0 = self.lookahead_1;
        self.lookahead_1 = self.token_stream.next();
    }

    pub fn node_id(&mut self) -> NodeId {
        self.id_allocator.allocate()
    }
}
