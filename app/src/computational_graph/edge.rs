use computational_graph::{NodeId};

pub type EdgeId = u16;
pub type Edges = Vec<Edge>;

#[derive(Debug)]
pub struct Edge {
    id: EdgeId,
    from_node: NodeId,
    to_node: NodeId,
}

impl Edge {
    pub fn new(id: EdgeId, from_node: NodeId, to_node: NodeId) -> Self {
        Edge { id, from_node, to_node }
    }
}