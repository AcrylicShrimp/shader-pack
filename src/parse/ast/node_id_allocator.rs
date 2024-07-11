use super::NodeId;

#[derive(Default, Debug, Clone, Hash)]
pub struct NodeIdAllocator {
    next_id: u32,
}

impl NodeIdAllocator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn allocate(&mut self) -> NodeId {
        self.next_id += 1;
        NodeId::new(self.next_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_allocator() {
        let mut allocator = NodeIdAllocator::new();
        let id1 = allocator.allocate();
        let id2 = allocator.allocate();
        assert_eq!(id1.get(), 1);
        assert_eq!(id2.get(), 2);
    }
}
