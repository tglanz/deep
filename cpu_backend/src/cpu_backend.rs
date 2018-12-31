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
pub struct TransientTensors;

pub struct CpuBackend {
    transients: TransientTensors
}

impl CpuBackend {
    pub fn new() -> Self {
        Self {
            transients: TransientTensors { }
        }
    }
}

impl Visitor for TransientTensors {
    fn before_visitations(&mut self) {
        println!("I wanna start allocating stuff for transeint tensors");
    }  

    fn visit_node(
        &mut self,
        node: &Node,
        input_edges: &[&Edge],
        output_edges: &[&Edge]
    ) {
        println!("visiting node:{:#?}", node);
    }

    fn after_visitations(&mut self) {
        println!("I should stop allocating stuff for transeint tensors, perhaps this is a good place for some sanity??");
    }
}

impl ForwardComputer for CpuBackend {

    type Initializer = TransientTensors;

    fn initializer<'a>(&'a mut self) -> &'a mut Self::Initializer {
        &mut self.transients
    }

    fn compute<'a>(&'a mut self, inputs: &'a HashMap<TensorId, Tensor<u16>>) {
        println!("i need to compute stuff");
    }

    fn collect<'a>(&'a self) -> HashMap<TensorId, Tensor<u16>> {
        HashMap::new()
    }
}