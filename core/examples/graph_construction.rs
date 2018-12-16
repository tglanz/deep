extern crate deep_core as core;

use core::{
    graph::{GraphBuilder, TensorDescriptor},
    operations::{
        add,
        dot,
        leaky_relu
    },
};

fn main() {
    // Notice that we don't specify as specific tensor in any stage of the graph

    let input_descriptor = TensorDescriptor::create(0).with_shape((3, ));
    let input_node_id = 0;

    let weights_descriptor = TensorDescriptor::create(1).with_shape((3, 3));
    let weights_node_id = 1;

    let bias_descriptor = TensorDescriptor::create(2).with_shape((3, ));
    let bias_node_id = 2;

    let dot_node_id = 1;
    let add_node_id = 2;
    let leaky_node_id = 3;

    /*
        the computation this graph does is still the same
        leaky(matrix*vector + bias)
    */
    let graph = GraphBuilder::create()
        .with_input(input_node_id, input_descriptor)
        .with_parameter(weights_node_id, weights_descriptor)
        .with_parameter(bias_node_id, bias_descriptor)
        .with_operation(dot_node_id, dot::operation())
        .with_operation(add_node_id, add::operation())
        .with_operation(leaky_node_id, leaky_relu::operation(0.57))
        .with_operand(0, input_node_id, dot_node_id, dot::operands::input())
        .with_operand(1, weights_node_id, dot_node_id, dot::operands::weights())
        .with_operand(2, dot_node_id, add_node_id, add::operands::input())
        .with_operand(3, bias_node_id, add_node_id, add::operands::input())
        .with_operand(4, add_node_id, leaky_node_id, leaky_relu::operands::input())
        .build(0);

    println!("{:#?}", graph);
}