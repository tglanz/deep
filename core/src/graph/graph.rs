use super::node::{Node, Nodes};
use super::edge::{Edge, Edges};

pub type GraphId = u16;

pub trait GraphVisitor {
    fn visit_node<'a>(node: &'a Node, input_edges: &[&'a Edge], output_edges: &[&'a Edge]);
}

#[derive(Debug)]
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
