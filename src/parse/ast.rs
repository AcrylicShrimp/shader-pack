mod node_id;
mod node_id_allocator;

pub use node_id::*;
pub use node_id_allocator::*;

use crate::{span::Span, symbol::Symbol};

use super::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstShaderPack {
    pub node_id: NodeId,
    pub span: Span,
    pub top_levels: Vec<AstTopLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstTopLevel {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstTopLevelKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstTopLevelKind {
    CompTime(AstCompTime<AstTopLevel>),
    FnDef(AstFnDef),
    Input(AstInput),
    Pass(AstPass),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstAttribute {
    pub node_id: NodeId,
    pub span: Span,
    pub items: Vec<AstAttributeItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstAttributeItem {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_at: AstPunc,
    pub ident: AstIdentifier,
    pub punc_assign: AstPunc,
    pub expr: AstStringLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTime<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_comptime: AstKeyword,
    pub kind: AstCompTimeKind<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstCompTimeKind<T> {
    If(AstCompTimeIf<T>),
    Loop(AstCompTimeLoop<T>),
}

/// Example:
///
/// - `comptime if ... { ... }`
/// - `comptime if ... { ... } else if ... { ... } else { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIf<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub if_part: Vec<AstCompTimeIfPart<T>>,
    pub else_if_parts: Vec<AstCompTimeElseIfPart<T>>,
    pub else_part: Option<AstCompTimeElsePart<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPart<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_if: AstKeyword,
    pub predicate: AstCompTimeIfPredicateExpr,
    pub block: AstCompTimeBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeElseIfPart<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_else: AstKeyword,
    pub keyword_if: AstKeyword,
    pub predicate: AstCompTimeIfPredicateExpr,
    pub block: AstCompTimeBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeElsePart<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_else: AstKeyword,
    pub block: AstCompTimeBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstCompTimeIfPredicateExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstCompTimeIfPredicateExprKind {
    Or(AstCompTimeIfPredicateExprOr),
    And(AstCompTimeIfPredicateExprAnd),
    Not(AstCompTimeIfPredicateExprNot),
    Flag(AstCompTimeIfPredicateExprFlag),
    Paren(AstCompTimeIfPredicateExprParen),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprOr {
    pub node_id: NodeId,
    pub span: Span,
    pub lhs: Box<AstCompTimeIfPredicateExpr>,
    pub keyword_or: AstKeyword,
    pub rhs: Box<AstCompTimeIfPredicateExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprAnd {
    pub node_id: NodeId,
    pub span: Span,
    pub lhs: Box<AstCompTimeIfPredicateExpr>,
    pub keyword_and: AstKeyword,
    pub rhs: Box<AstCompTimeIfPredicateExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprNot {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_not: AstKeyword,
    pub rhs: Box<AstCompTimeIfPredicateExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprFlag {
    pub node_id: NodeId,
    pub span: Span,
    pub flag: AstStringLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprParen {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_open_paren: AstPunc,
    pub expr: Box<AstCompTimeIfPredicateExpr>,
    pub punc_close_paren: AstPunc,
}

/// Example:
///
/// `comptime loop n times 1 + 1 { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeLoop<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_loop: AstKeyword,
    pub loop_var_ident: AstIdentifier,
    pub keyword_times: AstKeyword,
    pub expr: AstExpr,
    pub block: AstCompTimeBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeBlock<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_open_brace: AstPunc,
    pub items: Vec<T>,
    pub punc_close_brace: AstPunc,
}

/// Example:
///
/// `fn <identifier> (<params>) [<return-type>] { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnDef {
    pub node_id: NodeId,
    pub span: Span,
    pub attributes: Vec<AstAttribute>,
    pub keyword_fn: AstKeyword,
    pub ident: AstIdentifier,
    pub punc_open_paren: AstPunc,
    pub params: Vec<AstFnDefParam>,
    pub punc_close_paren: AstPunc,
    pub return_type: Option<AstFnDefReturnType>,
    pub punc_open_brace: AstPunc,
    pub statements: Vec<AstStatement>,
    pub punc_close_brace: AstPunc,
}

/// Example:
///
/// `<identifier> : <type-name> [,]`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnDefParam {
    pub node_id: NodeId,
    pub span: Span,
    pub attributes: Vec<AstAttribute>,
    pub ident: AstIdentifier,
    pub punc_colon: AstPunc,
    pub type_name: AstTypeName,
    pub punc_comma: Option<AstPunc>,
}

/// Example:
///
/// `-> <type-name>`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnDefReturnType {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_arrow: AstPunc,
    pub type_name: AstTypeName,
}

/// Example:
///
/// `in <identifier> : <type-name> ;`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstInput {
    pub node_id: NodeId,
    pub span: Span,
    pub attributes: Vec<AstAttribute>,
    pub keyword_in: AstKeyword,
    pub ident: AstIdentifier,
    pub punc_colon: AstPunc,
    pub type_name: AstTypeName,
    pub punc_semicolon: AstPunc,
}

/// Example:
///
/// `pass <identifier> { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstPass {
    pub node_id: NodeId,
    pub span: Span,
    pub attributes: Vec<AstAttribute>,
    pub keyword_pass: AstKeyword,
    pub ident: AstIdentifier,
    pub punc_open_brace: AstPunc,
    pub pass_levels: Vec<AstPassLevel>,
    pub punc_close_brace: AstPunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstPassLevel {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstPassLevelKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstPassLevelKind {
    Input(AstInput),
    Stage(AstStage),
}

/// Example:
///
/// `<stage> { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStage {
    pub node_id: NodeId,
    pub span: Span,
    pub attributes: Vec<AstAttribute>,
    pub stage: AstIdentifier,
    pub punc_open_brace: AstPunc,
    pub statements: Vec<AstStatement>,
    pub punc_close_brace: AstPunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStatement {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstStatementKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstStatementKind {
    VarDecl(AstStatementVarDecl),
    Assignment(AstStatementAssignment),
    Expr(AstExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStatementVarDecl {
    pub keyword_let: AstKeyword,
    pub ident: AstIdentifier,
    pub type_name: Option<AstStatementVarDeclTypeName>,
    pub assignment: Option<AstStatementVarDeclAssignment>,
    pub punc_semicolon: AstPunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStatementVarDeclTypeName {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_colon: AstPunc,
    pub type_name: AstTypeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStatementVarDeclAssignment {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_assignment: AstPunc,
    pub rhs: AstExpr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStatementAssignment {
    pub op: AstAssignmentOp,
    pub lhs: AstExpr,
    pub rhs: AstExpr,
    pub punc_semicolon: AstPunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstAssignmentOp {
    pub span_low: u32,
    pub kind: AstAssignmentOpKind,
}

impl AstAssignmentOp {
    pub fn span(self) -> Span {
        Span::new(self.span_low, self.span_low + self.kind.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstAssignmentOpKind {
    /// `=`
    Assign,
    /// `+=`
    AssignAdd,
    /// `-=`
    AssignSub,
    /// `*=`
    AssignMul,
    /// `/=`
    AssignDiv,
    /// `%=`
    AssignMod,
    /// `**=`
    AssignPow,
    /// `<<=`
    AssignShl,
    /// `>>=`
    AssignShr,
    /// `|=`
    AssignBitOr,
    /// `&=`
    AssignBitAnd,
    /// `^=`
    AssignBitXor,
}

impl AstAssignmentOpKind {
    pub fn len(self) -> u32 {
        match self {
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstExprKind,
    pub punc_semicolon: AstPunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstExprKind {
    Binary(AstBinaryExpr),
    Unary(AstUnaryExpr),
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstBinaryExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub op: AstBinaryExprOp,
    pub lhs: Box<AstExpr>,
    pub rhs: Box<AstExpr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstBinaryExprOp {
    pub span_low: u32,
    pub kind: AstBinaryExprOpKind,
}

impl AstBinaryExprOp {
    pub fn span(self) -> Span {
        Span::new(self.span_low, self.span_low + self.kind.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstBinaryExprOpKind {
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `**`
    Pow,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `|`
    BitOr,
    /// `&`
    BitAnd,
    /// `^`
    BitXor,
    /// `||`
    LogOr,
    /// `&&`
    LogAnd,
}

impl AstBinaryExprOpKind {
    pub fn len(self) -> u32 {
        match self {
            Self::Eq | Self::Ne | Self::Lt | Self::Ge => 2,
            Self::Gt | Self::Le => 1,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod => 1,
            Self::Pow => 2,
            Self::Shl | Self::Shr => 2,
            Self::BitOr | Self::BitAnd | Self::BitXor => 1,
            Self::LogOr | Self::LogAnd => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUnaryExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub op: AstUnaryExprOp,
    pub rhs: Box<AstExpr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstUnaryExprOp {
    pub span_low: u32,
    pub kind: AstUnaryExprOpKind,
}

impl AstUnaryExprOp {
    pub fn span(self) -> Span {
        Span::new(self.span_low, self.span_low + self.kind.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstUnaryExprOpKind {
    /// `+`
    Pos,
    /// `-`
    Neg,
    /// `!`
    LogNot,
    /// `~`
    BitNot,
}

impl AstUnaryExprOpKind {
    pub fn len(self) -> u32 {
        match self {
            Self::Pos | Self::Neg | Self::LogNot | Self::BitNot => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstPunc {
    pub span_low: u32,
    pub kind: AstPuncKind,
}

impl AstPunc {
    pub fn span(self) -> Span {
        Span::new(self.span_low, self.span_low + self.kind.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AstPuncKind {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Dot,
    Comma,
    Colon,
    Semicolon,
    At,
    Arrow,
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignPow,
    AssignShl,
    AssignShr,
    AssignBitOr,
    AssignBitAnd,
    AssignBitXor,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Shl,
    Shr,
    BitOr,
    BitAnd,
    BitXor,
    LogOr,
    LogAnd,
    BitNot,
    LogNot,
}

impl AstPuncKind {
    pub fn len(self) -> u32 {
        match self {
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
        }
    }

    pub fn into_token_kind(self) -> TokenKind {
        match self {
            AstPuncKind::OpenParen => TokenKind::OpenParen,
            AstPuncKind::CloseParen => TokenKind::CloseParen,
            AstPuncKind::OpenBrace => TokenKind::OpenBrace,
            AstPuncKind::CloseBrace => TokenKind::CloseBrace,
            AstPuncKind::OpenBracket => TokenKind::OpenBracket,
            AstPuncKind::CloseBracket => TokenKind::CloseBracket,
            AstPuncKind::Dot => TokenKind::Dot,
            AstPuncKind::Comma => TokenKind::Comma,
            AstPuncKind::Colon => TokenKind::Colon,
            AstPuncKind::Semicolon => TokenKind::Semicolon,
            AstPuncKind::At => TokenKind::At,
            AstPuncKind::Arrow => TokenKind::Arrow,
            AstPuncKind::Assign => TokenKind::Assign,
            AstPuncKind::AssignAdd => TokenKind::AssignAdd,
            AstPuncKind::AssignSub => TokenKind::AssignSub,
            AstPuncKind::AssignMul => TokenKind::AssignMul,
            AstPuncKind::AssignDiv => TokenKind::AssignDiv,
            AstPuncKind::AssignMod => TokenKind::AssignMod,
            AstPuncKind::AssignPow => TokenKind::AssignPow,
            AstPuncKind::AssignShl => TokenKind::AssignShl,
            AstPuncKind::AssignShr => TokenKind::AssignShr,
            AstPuncKind::AssignBitOr => TokenKind::AssignBitOr,
            AstPuncKind::AssignBitAnd => TokenKind::AssignBitAnd,
            AstPuncKind::AssignBitXor => TokenKind::AssignBitXor,
            AstPuncKind::Eq => TokenKind::Eq,
            AstPuncKind::Ne => TokenKind::Ne,
            AstPuncKind::Lt => TokenKind::Lt,
            AstPuncKind::Gt => TokenKind::Gt,
            AstPuncKind::Le => TokenKind::Le,
            AstPuncKind::Ge => TokenKind::Ge,
            AstPuncKind::Add => TokenKind::Add,
            AstPuncKind::Sub => TokenKind::Sub,
            AstPuncKind::Mul => TokenKind::Mul,
            AstPuncKind::Div => TokenKind::Div,
            AstPuncKind::Mod => TokenKind::Mod,
            AstPuncKind::Pow => TokenKind::Pow,
            AstPuncKind::Shl => TokenKind::Shl,
            AstPuncKind::Shr => TokenKind::Shr,
            AstPuncKind::BitOr => TokenKind::BitOr,
            AstPuncKind::BitAnd => TokenKind::BitAnd,
            AstPuncKind::BitXor => TokenKind::BitXor,
            AstPuncKind::LogOr => TokenKind::LogOr,
            AstPuncKind::LogAnd => TokenKind::LogAnd,
            AstPuncKind::BitNot => TokenKind::BitNot,
            AstPuncKind::LogNot => TokenKind::LogNot,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstKeyword {
    pub span_low: u32,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstIdentifier {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstIdentifierKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstIdentifierKind {
    Symbol(Symbol),
    Composed(AstComposedIdentifier),
}

/// Example:
///
/// `!ident("rule_str_{}", comptime_expr, ...)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstComposedIdentifier {
    pub punc_bang: AstPunc,
    pub keyword_ident: AstKeyword,
    pub punc_open_paren: AstPunc,
    pub rule_str: AstStringLiteral,
    pub punc_comma: Option<AstPunc>,
    pub args: Vec<AstComposedIdentifierArg>,
    pub punc_close_paren: AstPunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStringLiteral {
    pub node_id: NodeId,
    pub span: Span,
    pub content: Symbol,
    pub unquoted_content: Symbol,
    pub terminated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstComposedIdentifierArg {
    pub node_id: NodeId,
    pub span: Span,
    pub expr: AstExpr,
    pub punc_comma: Option<AstPunc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstTypeName {
    pub node_id: NodeId,
    pub span: Span,
    pub ident: AstIdentifier,
}
