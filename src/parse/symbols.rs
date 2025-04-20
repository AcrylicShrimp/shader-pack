use crate::symbol::Symbol;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SYMBOL_IDENT: Symbol = Symbol::from_str("ident");
}
