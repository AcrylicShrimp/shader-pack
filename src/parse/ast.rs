mod node_id;
mod node_id_allocator;

pub use node_id::*;
pub use node_id_allocator::*;

use super::lexer::TokenKind;
use crate::{span::Span, symbol::Symbol};

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
    Invalid,
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
    pub if_part: AstCompTimeIfPart<T>,
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
    Invalid,
    Single(AstCompTimeIfPredicateExprSingle),
    And(AstCompTimeIfPredicateExprAnd),
    Or(AstCompTimeIfPredicateExprOr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprSingle {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstCompTimeIfPredicateExprSingleKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstCompTimeIfPredicateExprSingleKind {
    Invalid,
    Flag(AstCompTimeIfPredicateExprFlag),
    Paren(AstCompTimeIfPredicateExprParen),
    Not(AstCompTimeIfPredicateExprNot),
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
pub struct AstCompTimeIfPredicateExprOr {
    pub node_id: NodeId,
    pub span: Span,
    pub lhs: Box<AstCompTimeIfPredicateExpr>,
    pub keyword_or: AstKeyword,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompTimeIfPredicateExprNot {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_not: AstKeyword,
    pub expr: Box<AstCompTimeIfPredicateExpr>,
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
    pub span: Span,
    pub kind: AstAssignmentOpKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstAssignmentOpKind {
    Invalid,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstExprKind {
    Invalid,
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
    pub span: Span,
    pub kind: AstBinaryExprOpKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstBinaryExprOpKind {
    Invalid,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUnaryExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub op: AstUnaryExprOp,
    pub rhs: Box<AstExpr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstUnaryExprOp {
    pub span: Span,
    pub kind: AstUnaryExprOpKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstUnaryExprOpKind {
    Invalid,
    /// `+`
    Pos,
    /// `-`
    Neg,
    /// `!`
    LogNot,
    /// `~`
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstPunc {
    pub span: Span,
    pub kind: AstPuncKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AstPuncKind {
    Invalid,
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
    pub fn matches_token_kind(self, token_kind: TokenKind) -> bool {
        match self {
            AstPuncKind::Invalid => false,
            AstPuncKind::OpenParen => token_kind == TokenKind::OpenParen,
            AstPuncKind::CloseParen => token_kind == TokenKind::CloseParen,
            AstPuncKind::OpenBrace => token_kind == TokenKind::OpenBrace,
            AstPuncKind::CloseBrace => token_kind == TokenKind::CloseBrace,
            AstPuncKind::OpenBracket => token_kind == TokenKind::OpenBracket,
            AstPuncKind::CloseBracket => token_kind == TokenKind::CloseBracket,
            AstPuncKind::Dot => token_kind == TokenKind::Dot,
            AstPuncKind::Comma => token_kind == TokenKind::Comma,
            AstPuncKind::Colon => token_kind == TokenKind::Colon,
            AstPuncKind::Semicolon => token_kind == TokenKind::Semicolon,
            AstPuncKind::At => token_kind == TokenKind::At,
            AstPuncKind::Arrow => token_kind == TokenKind::Arrow,
            AstPuncKind::Assign => token_kind == TokenKind::Assign,
            AstPuncKind::AssignAdd => token_kind == TokenKind::AssignAdd,
            AstPuncKind::AssignSub => token_kind == TokenKind::AssignSub,
            AstPuncKind::AssignMul => token_kind == TokenKind::AssignMul,
            AstPuncKind::AssignDiv => token_kind == TokenKind::AssignDiv,
            AstPuncKind::AssignMod => token_kind == TokenKind::AssignMod,
            AstPuncKind::AssignPow => token_kind == TokenKind::AssignPow,
            AstPuncKind::AssignShl => token_kind == TokenKind::AssignShl,
            AstPuncKind::AssignShr => token_kind == TokenKind::AssignShr,
            AstPuncKind::AssignBitOr => token_kind == TokenKind::AssignBitOr,
            AstPuncKind::AssignBitAnd => token_kind == TokenKind::AssignBitAnd,
            AstPuncKind::AssignBitXor => token_kind == TokenKind::AssignBitXor,
            AstPuncKind::Eq => token_kind == TokenKind::Eq,
            AstPuncKind::Ne => token_kind == TokenKind::Ne,
            AstPuncKind::Lt => token_kind == TokenKind::Lt,
            AstPuncKind::Gt => token_kind == TokenKind::Gt,
            AstPuncKind::Le => token_kind == TokenKind::Le,
            AstPuncKind::Ge => token_kind == TokenKind::Ge,
            AstPuncKind::Add => token_kind == TokenKind::Add,
            AstPuncKind::Sub => token_kind == TokenKind::Sub,
            AstPuncKind::Mul => token_kind == TokenKind::Mul,
            AstPuncKind::Div => token_kind == TokenKind::Div,
            AstPuncKind::Mod => token_kind == TokenKind::Mod,
            AstPuncKind::Pow => token_kind == TokenKind::Pow,
            AstPuncKind::Shl => token_kind == TokenKind::Shl,
            AstPuncKind::Shr => token_kind == TokenKind::Shr,
            AstPuncKind::BitOr => token_kind == TokenKind::BitOr,
            AstPuncKind::BitAnd => token_kind == TokenKind::BitAnd,
            AstPuncKind::BitXor => token_kind == TokenKind::BitXor,
            AstPuncKind::LogOr => token_kind == TokenKind::LogOr,
            AstPuncKind::LogAnd => token_kind == TokenKind::LogAnd,
            AstPuncKind::BitNot => token_kind == TokenKind::BitNot,
            AstPuncKind::LogNot => token_kind == TokenKind::LogNot,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstKeyword {
    pub span: Span,
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
    Invalid,
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
