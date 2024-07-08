use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(NonZeroU32);

impl Symbol {
    pub(crate) fn new(index: u32) -> Self {
        debug_assert_ne!(index, u32::MAX);
        Self(unsafe { NonZeroU32::new_unchecked(1 + index) })
    }

    pub fn index(self) -> u32 {
        self.0.get() - 1
    }
}

impl From<Symbol> for u32 {
    fn from(value: Symbol) -> Self {
        value.index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_index() -> u32 {
        rand::thread_rng().gen_range(0..u32::MAX - 1)
    }

    #[test]
    fn test_symbol_new() {
        let index = random_index();
        let symbol = Symbol::new(index);
        assert_eq!(symbol.index(), index);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_symbol_new_max() {
        Symbol::new(u32::MAX);
    }
}
