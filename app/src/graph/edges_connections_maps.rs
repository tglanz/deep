use std::collections::{HashMap};
use super::node::{NodeId, Nodes};
use super::edge::{Edges};

type Indices = Vec<usize>;

struct EdgesConnectionsMaps {
    indices_by_from: HashMap<NodeId, Indices>,
    indices_by_to: HashMap<NodeId, Indices>,
}

impl EdgesConnectionsMaps {
    pub fn create(nodes: &Nodes, edges: &Edges) -> EdgesConnectionsMaps {
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

        EdgesConnectionsMaps {
            indices_by_from,
            indices_by_to,
        }
    }
}