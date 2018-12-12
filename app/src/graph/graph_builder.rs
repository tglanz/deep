use super::graph::{Graph, GraphId};
use super::node::{Node, NodeId, Nodes};
use super::edge::{Edge, EdgeId, Edges};

use math::{Tensor};
use operations::{Operation, Operand};

#[derive(Debug)]
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

    pub fn with_input(self, id: NodeId, tensor: Tensor<u16>) -> Self {
        self.with_node(Node::InputNode(id, tensor))
    }

    pub fn with_parameter(self, id: NodeId, tensor: Tensor<u16>) -> Self {
        self.with_node(Node::ParameterNode(id, tensor))
    }

    pub fn with_operation(self, id: NodeId, operation: Operation) -> Self {
        self.with_node(Node::OperationNode(id, operation))
    }

    pub fn with_operand(self, id: EdgeId, from_node: NodeId, to_node: NodeId, operand: Operand) -> Self {
        self.with_edge(Edge::OperandEdge(id, (from_node, to_node), operand))
    }

    pub fn build(self, id: GraphId) -> Graph {
        Graph::new(id, self.nodes, self.edges)
    }
}