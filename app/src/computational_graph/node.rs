use operations::{Operation};

pub type NodeId = u16;
pub type Nodes = Vec<Node>;

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    operation: Operation,
}

impl Node {
    pub fn new(id: NodeId, operation: Operation) -> Self {
        Node {
            id, operation
        }
    }
}