
extern crate serde;
extern crate serde_json;

extern crate deep;
use deep::prelude::*;

use std::fs;

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
        .with_parameter(bias_node_id, &bias_descriptor)
        .with_operation(add_node_id, add::operation())
        .with_operation(leaky_node_id, leaky_relu::operation(0.57))
        .with_operation(dot_node_id, dot::operation())
        .with_operand(7, input_node_id, dot_node_id, dot::operands::INPUT)
        .with_operand(8, weights_node_id, dot_node_id, dot::operands::WEIGHTS)
        .with_operand(9, dot_node_id, add_node_id, add::operands::INPUT)
        .with_operand(10, bias_node_id, add_node_id, add::operands::INPUT)
        .with_operand(11, add_node_id, leaky_node_id, leaky_relu::operands::INPUT)
        .build(0)
}


fn main() -> std::io::Result<()> {
    
    let input_descriptor = TensorDescriptor::create(0, (3, ));
    let weights_descriptor = TensorDescriptor::create(1, (3, 3));
    let bias_descriptor = TensorDescriptor::create(2, (3, ));
    let graph = construct_graph(&input_descriptor, &weights_descriptor, &bias_descriptor);

    let serialized = serde_json::to_string_pretty(&graph).unwrap();
    fs::write("graph.json", serialized).expect("Unable to write serialized graph content (graph.json)");

    let content = fs::read_to_string("graph.json").expect("Unable to read deserialized graph content (graph.json)");
    let deserialized: Graph = serde_json::from_str(&content).unwrap();
    println!("Deserialized graph {:#?}", deserialized);

    Ok(())
}