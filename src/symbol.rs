mod chunk;
mod interner;
mod symbol;

pub use symbol::*;

use self::interner::Interner;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::fmt::{Debug, Display};

lazy_static! {
    static ref STR_INTERNER: Mutex<Interner> = Mutex::new(Interner::new());
}

impl Symbol {
    pub fn from_str(str: impl AsRef<str>) -> Self {
        STR_INTERNER.lock().intern(str)
    }

    pub fn to_str(self) -> &'static str {
        STR_INTERNER.lock().str(self)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.to_str())
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.to_str())
    }
}

impl From<Symbol> for &'static str {
    fn from(symbol: Symbol) -> Self {
        symbol.to_str()
    }
}

impl From<Symbol> for String {
    fn from(symbol: Symbol) -> Self {
        symbol.to_str().to_owned()
    }
}

impl From<&'static str> for Symbol {
    fn from(str: &'static str) -> Self {
        Symbol::from_str(str)
    }
}

impl From<String> for Symbol {
    fn from(string: String) -> Self {
        Symbol::from_str(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Alphanumeric, Rng};

    fn random_str(len: usize) -> String {
        let mut rng = rand::thread_rng();
        String::from_iter((0..len).map(|_| rng.sample(Alphanumeric) as char))
    }

    fn symbol_index() -> u32 {
        rand::thread_rng().gen_range(1..=u32::MAX / 2)
    }

    #[test]
    fn test_symbol_from_str_and_to_str_equals() {
        let symbol = Symbol::from_str("test");
        assert_eq!(symbol.to_str(), "test");

        let symbol = Symbol::from_str("");
        assert_eq!(symbol.to_str(), "");

        let str = random_str(Interner::CHUNK_SIZE * 2);
        let symbol = Symbol::from_str(&str);
        assert_eq!(symbol.to_str(), &str);
    }

    #[test]
    fn test_symbol_index_equals() {
        let symbol = Symbol::from_str("test");
        let symbol2 = Symbol::from_str("test");
        assert_eq!(symbol.index(), symbol2.index());
    }

    #[test]
    #[should_panic]
    fn test_symbol_to_str_invalid_index() {
        Symbol::new(symbol_index()).to_str();
    }
}
