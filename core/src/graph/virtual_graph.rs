use std::collections::{HashMap};
use std::iter::{Iterator};

use super::{
    graph::Graph,
    edges_indices::EdgesIndices,
    node::{Node, NodeId},
    edge::{Edge},
};

/// The aim of this struct is to provide model a given graph with mechanisms of manipulations,
/// without mutating the source graph
/// 
/// It is used for algorithms of sorts and optimizers
pub struct VirtualGraph<'a> {
    index_to_node: HashMap<usize, &'a Node>,
    index_to_edge: HashMap<usize, &'a Edge>,
    source_node_id_to_index: HashMap<NodeId, usize>,
    source_edges_indices: EdgesIndices,
}

impl<'a> VirtualGraph<'a> {

    /// This is the objectives of this model. To be based upon an existing graph,
    /// with an existing lifetime, but never actually change it
    /// 
    /// The source_* fields are not changing, they reflect the global data as acquired from the source
    /// of the virtualization
    /// 
    /// The nodes/edges that exists in index_to_* fields are teh fields that determine their existance in
    /// the virtualized graph
    pub fn from_graph(graph: &'a Graph) -> Self {
        let nodes = graph.get_nodes();
        let edges = graph.get_edges();

        let index_to_node = nodes.iter().enumerate().map(|(idx, node)| (idx, node)).collect();
        let index_to_edge = edges.iter().enumerate().map(|(idx, edge)| (idx, edge)).collect();
        let source_node_id_to_index = nodes.iter().enumerate().map(|(idx, node)| (node.get_id(), idx)).collect();

        Self {
            index_to_node,
            index_to_edge,
            source_node_id_to_index,
            source_edges_indices: EdgesIndices::from_graph(graph),
        }
    }

    pub fn get_index_of_source_node(&self, node_id: &NodeId) -> Option<&usize> {
        self.source_node_id_to_index.get(node_id)
    }

    pub fn get_orphan_nodes_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        for (index, node) in self.index_to_node.iter() {
            if self.source_edges_indices.get_indices_to(node.get_id()).is_none() {
                indices.push(*index);
            }
        }

        indices
    }

    pub fn get_indices_of_edges_into(&self, index: usize) -> Vec<usize> {
        let mut indices = Vec::new();        
        if let Some(node) = self.get_node(index) {
            let node_id = node.get_id();
            if let Some(edges_indices) = self.source_edges_indices.get_indices_to(node_id) {
                for edge_index in edges_indices {
                    if self.index_to_edge.contains_key(edge_index) {
                        indices.push(*edge_index);
                    }
                }
            }
        }

        indices
    }
    
    pub fn get_indices_of_edges_from(&self, index: usize) -> Vec<usize> {
        let mut indices = Vec::new();        
        if let Some(node) = self.get_node(index) {
            let node_id = node.get_id();
            if let Some(edges_indices) = self.source_edges_indices.get_indices_from(node_id) {
                for edge_index in edges_indices {
                    if self.index_to_edge.contains_key(edge_index) {
                        indices.push(*edge_index);
                    }
                }
            }
        }

        indices
    }

    pub fn get_nodes(&self) -> impl Iterator<Item=(&usize, &&'a Node)> {
        self.index_to_node.iter()
    }

    pub fn get_node(&self, index: usize) -> Option<&&'a Node> {
        self.index_to_node.get(&index)
    }

    pub fn get_edges(&self) -> impl Iterator<Item=(&usize, &&'a Edge)> {
        self.index_to_edge.iter()
    }

    pub fn get_edge(&self, index: usize) -> Option<&&'a Edge> {
        self.index_to_edge.get(&index)
    }

    pub fn remove_edge(&mut self, index: usize) -> Option<&'a Edge> {
        self.index_to_edge.remove(&index)
    }

    pub fn remove_node(&mut self, index: usize) -> Option<&'a Node> {
        self.index_to_node.remove(&index)
    }
}
