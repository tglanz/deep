extern crate app;

use app::{
    math::{Tensor},
    graph::{GraphBuilder},
    graph::Node::{
        InputNode as Input,
        ParameterNode as Parameter,
        OperationNode as Operation,
    },
    graph::Edge::{
        OperandEdge as Operand,
    },
    operations::{
        add,
        dot,
        leaky_relu
    }
};

fn main() {
    /*
        the setup is quite verbose - 

        it's ok, it is not intended to be an api, but as a framework.
        later, an api for the graph will be provided.

        for now, we try to keep it as simple as possible, with enough clarity and
        ease of data access to work with.

        in addition, we try to leverage ownership mechanisms to keep the data safe from smurfs;
        (try changing stuff; keeping the graph in variable and remodify, etc...)
    */
    let input: Tensor<u16> = Tensor::default((3,));
    let input_node_id = 0;

    let weights: Tensor<u16> = Tensor::default((3, 3));
    let weights_node_id = 1;

    let bias: Tensor<u16> = Tensor::default((3,));
    let bias_node_id = 2;

    let dot_node_id = 1;
    let add_node_id = 2;
    let leaky_node_id = 3;

    /*
        the computation this graph does is
        leaky(matrix*vector + bias)
    */
    let graph = GraphBuilder::create()
        .with_node(Input(input_node_id, input))
        .with_node(Parameter(weights_node_id, weights))
        .with_node(Parameter(bias_node_id, bias))
        .with_node(Operation(dot_node_id, dot::operation()))
        .with_node(Operation(add_node_id, add::operation()))
        .with_node(Operation(leaky_node_id, leaky_relu::operation(0.57)))
        .with_edge(Operand(0, (input_node_id, dot_node_id), dot::operands::input()))
        .with_edge(Operand(1, (weights_node_id, dot_node_id), dot::operands::weights()))
        .with_edge(Operand(2, (dot_node_id, add_node_id), add::operands::input()))
        .with_edge(Operand(3, (bias_node_id, add_node_id), add::operands::input()))
        .with_edge(Operand(4, (add_node_id, leaky_node_id), leaky_relu::operands::input()))
        .build(0);

    // just print the graph, looks kinda nice with #, just wanna lower_case and remove types
    // an alternative is to use serde
    println!("{:#?}", graph);
}