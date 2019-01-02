extern crate deep;

use std::collections::{HashMap};
use deep::prelude::*;

fn construct_graph(input_descriptor: &TensorDescriptor,
                   weights_descriptor: &TensorDescriptor,
                   bias_descriptor: &TensorDescriptor) -> Graph {
    // Some Ids 
    let input_node_id = 0;
    let weights_node_id = 1;
    let bias_node_id = 2;

    let dot_node_id = 3;
    let add_node_id = 4;
    let leaky_node_id = 5;

    /*
        the computation this graph does is still the same
        leaky(matrix*vector + bias)
    */
    GraphBuilder::create()
        .with_input(input_node_id, &input_descriptor)
        .with_parameter(weights_node_id, &weights_descriptor)
        .with_operand(9, dot_node_id, add_node_id, add::operands::INPUT)
        .with_operation(add_node_id, add::operation())
        .with_parameter(bias_node_id, &bias_descriptor)
        .with_operation(leaky_node_id, leaky_relu::operation(0.57))
        .with_operand(11, add_node_id, leaky_node_id, leaky_relu::operands::INPUT)
        .with_operand(7, input_node_id, dot_node_id, dot::operands::INPUT)
        .with_operand(8, weights_node_id, dot_node_id, dot::operands::WEIGHTS)
        .with_operation(dot_node_id, dot::operation())
        .with_operand(10, bias_node_id, add_node_id, add::operands::INPUT)
        .build(0)
}

fn main(){
    
    let input_descriptor = TensorDescriptor::create(0, (3, ));
    let weights_descriptor = TensorDescriptor::create(1, (3, 3));
    let bias_descriptor = TensorDescriptor::create(2, (3, ));
    let graph = construct_graph(&input_descriptor, &weights_descriptor, &bias_descriptor);

    let mut backend = CpuBackend::new();
    let mut initializer = backend.create_initializer();

    // Initialize the backend
    {
        let mut iterator = TopologicalIterator::from_graph(&graph);
        Traverser::from_graph(&graph).traverse(&mut initializer, &mut iterator);
    }

    // Forward Computation
    {
        let mut inputs = HashMap::new();
        inputs.insert(input_descriptor.get_tensor_id(), Tensor::value(3, input_descriptor.get_shape().clone()));
        inputs.insert(weights_descriptor.get_tensor_id(), Tensor::value(4, weights_descriptor.get_shape().clone()));
        inputs.insert(bias_descriptor.get_tensor_id(), Tensor::value(1, bias_descriptor.get_shape().clone()));
        backend.compute(initializer, inputs);
    }
}