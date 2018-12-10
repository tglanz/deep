use super::node::{NodeId};

pub type EdgeId = u16;
pub type Edges = Vec<Edge>;
pub type EdgeConnection = (NodeId, NodeId);

#[derive(Debug)]
pub struct Edge {
    id: EdgeId,
    connection: EdgeConnection,
}

impl Edge {
    pub fn new(id: EdgeId, from_node: NodeId, to_node: NodeId) -> Self {
        Edge {
            id,
            connection: (from_node, to_node)
        }
    }

    pub fn get_id(&self) -> EdgeId {
        self.id
    }

    pub fn get_connection(&self) -> (NodeId, NodeId) {
        self.connection
    }
}