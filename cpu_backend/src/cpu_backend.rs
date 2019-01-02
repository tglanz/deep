use std::collections::{HashMap};

use core::{
    Tensor,
    graph::{
        node::Node,
        edge::Edge,
        TensorId,
        traversal::{
            Visitor,
        },
    },
    backend::{
        forward::{
            ForwardComputer
        }
    },
};

#[derive(Debug)]
pub struct Initializer<'a> {
    nodes_sequence: Vec<&'a Node>,
    inputs_sequence: Vec<Vec<&'a Edge>>,
    outputs_sequence: Vec<Vec<&'a Edge>>
}

impl<'a> Initializer<'a> {
    fn new() -> Self {
        Self {
            nodes_sequence: Vec::new(),
            inputs_sequence: Vec::new(),
            outputs_sequence: Vec::new()
        }
    }
}

pub struct CpuBackend {
}

impl CpuBackend {
    pub fn new() -> Self {
        Self { }
    }
}

impl<'a> Visitor<'a> for Initializer<'a> {
    fn before_visitations(&mut self) {
        println!("Initializer statrting to initalize stuff");
    }  

    fn visit_node(
        &mut self,
        node: &'a Node,
        input_edges: &[&'a Edge],
        output_edges: &[&'a Edge]
    ) {
        let mut inputs = Vec::new();
        for edge in input_edges {
            inputs.push(*edge);
        }

        let mut outputs = Vec::new();
        for edge in output_edges {
            outputs.push(*edge);
        }

        self.nodes_sequence.push(node);
        self.inputs_sequence.push(inputs);
        self.outputs_sequence.push(outputs);
    }

    fn after_visitations(&mut self) {
        println!("Initializer is done initializing stuff");
    }
}

impl<'a> ForwardComputer<'a> for CpuBackend {

    type Initializer = Initializer<'a>;

    fn create_initializer(&mut self) -> Self::Initializer {
        Initializer::new()
    }

    fn compute(&mut self, initializer: Self::Initializer, _inputs: HashMap<TensorId, Tensor<u16>>) {
        for (index, node) in initializer.nodes_sequence.iter().enumerate() {
            let current_inputs = &initializer.inputs_sequence[index];
            let current_outputs = &initializer.outputs_sequence[index];

            match node {
                Node::ParameterNode { .. } => {
                    println!("Got a parameter node, i don't think there is something to be done");
                },
                Node::InputNode { .. } => {
                    println!("Got an input node, i don't think there is something to be done");
                },
                Node::OperationNode { operation, .. } => {
                    println!("Got an operation node, finally there is something that should be done!");
                    println!(" - Operation: {:#?}", operation);
                    println!(" - Inputs:");
                    for edge in current_inputs {
                        match edge {
                            Edge::OperandEdge { operand, .. } => {
                                println!("    - Operand: {:#?}", operand);
                            }
                        }
                    }
                    println!(" - Outputs:");
                    for edge in current_outputs {
                        match edge {
                            Edge::OperandEdge { operand, .. } => {
                                println!("    - Operand: {:#?}", operand);
                            }
                        }
                    }
                }
            }
        }
    }

    fn collect(&self) -> HashMap<TensorId, Tensor<u16>> {
        HashMap::new()
    }
}