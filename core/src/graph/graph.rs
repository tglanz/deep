use super::node::{Nodes};
use super::edge::{Edges};

pub type GraphId = u16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    id: GraphId,
    nodes: Nodes,
    edges: Edges,
}

impl Graph {
    pub fn new(id: GraphId, nodes: Nodes, edges: Edges) -> Graph {
        Graph {
            id,
            nodes, 
            edges
        }
    }

    pub fn get_nodes(&self) -> &Nodes {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Edges {
        &self.edges
    }
}
