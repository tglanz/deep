use computational_graph::{NodeId, UniformId};

pub type EdgeId = u16;
pub type Edges = Vec<Edge>;

#[derive(Debug)]
pub enum Edge {
    NodeNodeEdge(EdgeId, (NodeId, NodeId)),
    UniformNodeEdge(EdgeId, (UniformId, NodeId)),
}