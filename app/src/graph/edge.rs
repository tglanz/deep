use super::node::{NodeId};
use operations::{Operand};

pub type EdgeId = u16;
pub type Edges = Vec<Edge>;
pub type EdgeConnection = (NodeId, NodeId);

#[derive(Debug)]
pub enum Edge {
    OperandEdge (EdgeId, EdgeConnection, Operand)
}

impl Edge {
    pub fn get_id(&self) -> EdgeId {
        match *self {
            Edge::OperandEdge(id, ..) => id,
        }
    }

    pub fn get_connection(&self) -> EdgeConnection {
        match *self {
            Edge::OperandEdge(_, connection, ..) => connection,
        }
    }
}