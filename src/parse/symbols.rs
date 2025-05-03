use crate::symbol::Symbol;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SYMBOL_IDENT: Symbol = Symbol::from_str("ident");
    pub static ref SYMBOL_COMPTIME: Symbol = Symbol::from_str("comptime");
    pub static ref SYMBOL_IF: Symbol = Symbol::from_str("if");
    pub static ref SYMBOL_ELSE: Symbol = Symbol::from_str("else");
    pub static ref SYMBOL_LOOP: Symbol = Symbol::from_str("loop");
    pub static ref SYMBOL_TIMES: Symbol = Symbol::from_str("times");
    pub static ref SYMBOL_OR: Symbol = Symbol::from_str("or");
    pub static ref SYMBOL_AND: Symbol = Symbol::from_str("and");
    pub static ref SYMBOL_NOT: Symbol = Symbol::from_str("not");
}
