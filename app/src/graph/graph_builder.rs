use super::graph::{Graph, GraphId};
use super::node::{Node, Nodes};
use super::edge::{Edge, Edges};

pub struct GraphBuilder {
    nodes: Nodes,
    edges: Edges,
}

impl GraphBuilder {
    pub fn create() -> Self {
        GraphBuilder {
            nodes: Nodes::new(),
            edges: Edges::new(),
        }
    }

    pub fn with_node(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edges.push(edge);
        self
    }

    pub fn build(self, id: GraphId) -> Graph {
        Graph::new(id, self.nodes, self.edges)
    }
}