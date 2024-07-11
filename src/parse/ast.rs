mod node_id;
mod node_id_allocator;

pub use node_id::*;
pub use node_id_allocator::*;

use super::lexer::Token;
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
    Compile(AstCompile<AstTopLevel>),
    FnDef(AstFnDef),
    Input(AstInput),
    Pass(AstPass),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompile<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_compile: AstKeyword,
    pub kind: AstCompileKind<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstCompileKind<T> {
    If(AstCompileIf<T>),
    Loop(AstCompileLoop<T>),
}

/// Example:
///
/// - `compile if (...) { ... }`
/// - `compile if (...) { ... } else if (...) { ... } else { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIf<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub if_part: Vec<AstCompileIfPart<T>>,
    pub else_if_parts: Vec<AstCompileElseIfPart<T>>,
    pub else_part: Option<AstCompileElsePart<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPart<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_if: AstKeyword,
    pub predicate: AstCompileIfPredicateExpr,
    pub block: AstCompileBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileElseIfPart<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_else: AstKeyword,
    pub keyword_if: AstKeyword,
    pub predicate: AstCompileIfPredicateExpr,
    pub block: AstCompileBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileElsePart<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_else: AstKeyword,
    pub block: AstCompileBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPredicateExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstCompileIfPredicateExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstCompileIfPredicateExprKind {
    Or(AstCompileIfPredicateExprOr),
    And(AstCompileIfPredicateExprAnd),
    Not(AstCompileIfPredicateExprNot),
    Flag(AstCompileIfPredicateExprFlag),
    Paren(AstCompileIfPredicateExprParen),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPredicateExprOr {
    pub node_id: NodeId,
    pub span: Span,
    pub lhs: Box<AstCompileIfPredicateExpr>,
    pub keyword_or: AstKeyword,
    pub rhs: Box<AstCompileIfPredicateExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPredicateExprAnd {
    pub node_id: NodeId,
    pub span: Span,
    pub lhs: Box<AstCompileIfPredicateExpr>,
    pub keyword_and: AstKeyword,
    pub rhs: Box<AstCompileIfPredicateExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPredicateExprNot {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_not: AstKeyword,
    pub rhs: Box<AstCompileIfPredicateExpr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPredicateExprFlag {
    pub node_id: NodeId,
    pub span: Span,
    pub flag: AstStringLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileIfPredicateExprParen {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_open_paren: Token,
    pub expr: Box<AstCompileIfPredicateExpr>,
    pub punc_close_paren: Token,
}

/// Example:
///
/// `compile loop n times 1 + 1 { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileLoop<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_loop: AstKeyword,
    pub loop_var_ident: AstIdentifier,
    pub keyword_times: AstKeyword,
    pub expr: AstExpr,
    pub block: AstCompileBlock<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstCompileBlock<T> {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_open_brace: Token,
    pub items: Vec<T>,
    pub punc_close_brace: Token,
}

/// Example:
///
/// `fn <identifier> (<params>) [<return-type>] { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnDef {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_fn: AstKeyword,
    pub ident: AstIdentifier,
    pub punc_open_paren: Token,
    pub params: Vec<AstFnDefParam>,
    pub punc_close_paren: Token,
    pub return_type: Option<AstFnDefReturnType>,
    pub punc_open_brace: Token,
    pub statements: Vec<AstStatement>,
    pub punc_close_brace: Token,
}

/// Example:
///
/// `<identifier> : <type-name> [,]`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnDefParam {
    pub node_id: NodeId,
    pub span: Span,
    pub ident: AstIdentifier,
    pub punc_colon: Token,
    pub type_name: AstTypeName,
    pub punc_comma: Option<Token>,
}

/// Example:
///
/// `-> <type-name>`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnDefReturnType {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_arrow: Token,
    pub type_name: AstTypeName,
}

/// Example:
///
/// `in <identifier> : <type-name> ;`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstInput {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_in: AstKeyword,
    pub ident: AstIdentifier,
    pub punc_colon: Token,
    pub type_name: AstTypeName,
    pub punc_semicolon: Token,
}

/// Example:
///
/// `pass <identifier> { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstPass {
    pub node_id: NodeId,
    pub span: Span,
    pub keyword_pass: AstKeyword,
    pub ident: AstIdentifier,
    pub punc_open_brace: Token,
    pub pass_levels: Vec<AstPassLevel>,
    pub punc_close_brace: Token,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstPassLevel {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstPassLevelKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstPassLevelKind {
    Stage(AstStage),
    // TODO
}

/// Example:
///
/// `<stage> { ... }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStage {
    pub node_id: NodeId,
    pub span: Span,
    pub stage: AstIdentifier,
    pub punc_open_brace: Token,
    pub statements: Vec<AstStatement>,
    pub punc_close_brace: Token,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStatement {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstStatementKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstStatementKind {
    VarDecl(AstStateLevelVarDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstStateLevelVarDecl {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstExpr {
    pub node_id: NodeId,
    pub span: Span,
    pub kind: AstExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstExprKind {
    Binary(AstBinaryExpr),
    Unary(AstUnaryExpr),
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstBinaryExpr {
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUnaryExpr {
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstKeyword {
    pub node_id: NodeId,
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
    Simple(AstSimpleIdentifier),
    Composed(AstComposedIdentifier),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstSimpleIdentifier {
    pub node_id: NodeId,
    pub span: Span,
    pub symbol: Symbol,
}

/// Example:
///
/// `!ident("rule_str_{}", comptime_expr, ...)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstComposedIdentifier {
    pub node_id: NodeId,
    pub span: Span,
    pub punc_bang: Token,
    pub keyword_ident: Token,
    pub punc_open_paren: Token,
    pub rule_str: AstStringLiteral,
    pub punc_comma: Option<Token>,
    pub args: Vec<AstComposedIdentifierArg>,
    pub punc_close_paren: Token,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstComposedIdentifierArg {
    pub node_id: NodeId,
    pub span: Span,
    pub expr: AstExpr,
    pub punc_comma: Option<Token>,
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
pub struct AstTypeName {
    pub node_id: NodeId,
    pub span: Span,
    pub ident: AstIdentifier,
}
