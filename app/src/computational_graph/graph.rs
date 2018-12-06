use computational_graph::*;

pub type GraphId = u16;

#[derive(Debug)]
pub struct Graph {
    id: GraphId,
    nodes: Nodes,
    edges: Edges,
}

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
        Graph {
            id,
            nodes: self.nodes,
            edges: self.edges
        }
    }
}