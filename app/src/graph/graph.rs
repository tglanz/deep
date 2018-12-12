use std::collections::HashMap;

use super::node::{Node, NodeId, Nodes};
use super::edge::{Edge, Edges};

pub type GraphId = u16;
type Indices = Vec<usize>;

pub trait GraphVisitor {
    fn visit_node<'a>(node: &'a Node, input_edges: &[&'a Edge], output_edges: &[&'a Edge]);
}

#[derive(Debug)]
pub struct GraphObjects {
    nodes: Nodes,
    edges: Edges,
}

#[derive(Debug)]
pub struct Graph {
    id: GraphId,
    objects: GraphObjects,
    edges_indices: EdgesIndices,
}

impl Graph {
    pub fn new(id: GraphId, nodes: Nodes, edges: Edges) -> Graph {
        let edges_indices = EdgesIndices::create(&nodes, &edges);
        let objects = GraphObjects { nodes, edges };
        Graph { id, objects, edges_indices }
    }

    pub fn get_nodes(&self) -> &Nodes {
        &self.objects.nodes
    }

    pub fn get_edges(&self) -> &Edges {
        &self.objects.edges
    }

    pub fn get_edges_indices(&self) -> &EdgesIndices {
        &self.edges_indices
    }
}

#[derive(Debug)]
pub struct EdgesIndices {
    indices_by_from: HashMap<NodeId, Indices>,
    indices_by_to: HashMap<NodeId, Indices>,
}

impl EdgesIndices {
    pub fn create(nodes: &Nodes, edges: &Edges) -> Self {
        let mut indices_by_from = HashMap::new();
        let mut indices_by_to = HashMap::new();

        for node in nodes {
            let node_id = node.get_id();

            let mut indices_from_node = Vec::new();
            let mut indices_to_node = Vec::new();

            for (edge_index, edge) in edges.iter().enumerate() {
                let (from_node_id, to_node_id) = edge.get_connection();

                if from_node_id == node_id {
                    indices_from_node.push(edge_index);
                }

                if to_node_id == node_id {
                    indices_to_node.push(edge_index);
                }
            }

            if indices_from_node.len() > 0 {
                indices_by_from.insert(node_id, indices_from_node);
            }

            if indices_to_node.len() > 0 {
                indices_by_to.insert(node_id, indices_to_node);
            }
        }

        EdgesIndices {
            indices_by_from,
            indices_by_to,
        }
    }
}