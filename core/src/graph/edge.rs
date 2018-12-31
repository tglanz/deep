use super::node::{NodeId};
use operations::{Operand};

pub type EdgeId = u16;
pub type Edges = Vec<Edge>;
pub type EdgeConnection = (NodeId, NodeId);

#[derive(Serialize, Deserialize, Debug)]
pub enum Edge {
    OperandEdge { id: EdgeId, connection: EdgeConnection, operand: Operand }
}

impl Edge {
    pub fn get_id(&self) -> EdgeId {
        match *self {
            Edge::OperandEdge { id, .. } => id,
        }
    }

    pub fn get_connection(&self) -> EdgeConnection {
        match *self {
            Edge::OperandEdge { connection, .. } => connection,
        }
    }
}