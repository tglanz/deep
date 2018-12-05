use computational_graph::*;

pub type GraphId = u16;

#[derive(Debug)]
pub struct Graph {
    id: GraphId,
    uniforms: Uniforms,
    nodes: Nodes,
    edges: Edges,
}

impl Graph {
    pub fn new(id: GraphId) -> Self {
        Graph {
            id,
            uniforms: Uniforms::new(),
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

    pub fn with_uniform(mut self, uniform: Uniform) -> Self {
        self.uniforms.push(uniform);
        self
    }
}