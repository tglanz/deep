use operations::{Operation, Operand};

use super::{
    graph::{
        Graph, GraphId,
    },
    node::{
        NodeId, Node, Nodes,
    },
    edge::{
        EdgeId, Edge, Edges,
    },
    tensor_descriptor::{
        TensorDescriptor,
    },
};

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

    pub fn with_input(self, id: NodeId, tensor_descriptor: TensorDescriptor) -> Self {
        self.with_node(Node::InputNode { id, tensor_descriptor })
    }

    pub fn with_parameter(self, id: NodeId, tensor_descriptor: TensorDescriptor) -> Self {
        self.with_node(Node::ParameterNode { id, tensor_descriptor })
    }

    pub fn with_operation(self, id: NodeId, operation: Operation) -> Self {
        self.with_node(Node::OperationNode { id, operation })
    }

    pub fn with_operand(self, id: EdgeId, from_node: NodeId, to_node: NodeId, operand: Operand) -> Self {
        self.with_edge(Edge::OperandEdge {
            id, 
            connection: (from_node, to_node),
            operand
        })
    }

    pub fn build(self, id: GraphId) -> Graph {
        Graph::new(id, self.nodes, self.edges)
    }
}