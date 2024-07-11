use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(NonZeroU32);

impl NodeId {
    pub fn new(id: u32) -> Self {
        debug_assert_ne!(id, 0);
        Self(unsafe { NonZeroU32::new_unchecked(id) })
    }

    pub fn get(self) -> u32 {
        self.0.get()
    }
}
