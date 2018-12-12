extern crate app;

// Notice this `use` against the `use` in the file graph_api_1.rs 
use app::{
    math::{Tensor},
    graph::{GraphBuilder},
    operations::{
        add,
        dot,
        leaky_relu
    },
};

fn main() {
    /* the setup is a bit less verbose than before; see graph_api_1 */
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
        the computation this graph does is still the same
        leaky(matrix*vector + bias)
    */
    let graph = GraphBuilder::create()
        .with_input(input_node_id, input)
        .with_parameter(weights_node_id, weights)
        .with_parameter(bias_node_id, bias)
        .with_operation(dot_node_id, dot::operation())
        .with_operation(add_node_id, add::operation())
        .with_operation(leaky_node_id, leaky_relu::operation(0.57))
        .with_operand(0, input_node_id, dot_node_id, dot::operands::input())
        .with_operand(1, weights_node_id, dot_node_id, dot::operands::weights())
        .with_operand(2, dot_node_id, add_node_id, add::operands::input())
        .with_operand(3, bias_node_id, add_node_id, add::operands::input())
        .with_operand(4, add_node_id, leaky_node_id, leaky_relu::operands::input())
        .build(0);

    // just print the graph, looks kinda nice with #, just wanna lower_case and remove types
    // an alternative is to use serde
    println!("{:#?}", graph);
}