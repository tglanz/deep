use std::collections::HashMap;

use super::graph::{Graph};
use super::node::{Nodes, NodeId};
use super::edge::{Edges};

pub type Indices = Vec<usize>;

#[derive(Debug)]
pub struct EdgesIndices {
    indices_by_from: HashMap<NodeId, Indices>,
    indices_by_to: HashMap<NodeId, Indices>,
}

impl EdgesIndices {

    pub fn get_indices_from(&self, node_id: NodeId) -> Option<&Indices> {
        self.indices_by_from.get(&node_id)
    }

    pub fn get_indices_to(&self, node_id: NodeId) -> Option<&Indices> {
        self.indices_by_to.get(&node_id)
    }

    pub fn from_graph(graph: &Graph) -> Self {
        Self::create(graph.get_nodes(), graph.get_edges())
    }

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