#![allow(dead_code)]
#![allow(unused_imports)]

mod math;
mod operations;
mod computational_graph;

fn tensor_into_iterable() {
    use math::Tensor;
    let tensor: Tensor<u16> = Tensor::default((3, 3, 3));
    for item in &tensor {
        println!("tensor[{:?}] == {:?}", item.index, item.value);
    }
}

fn tensor_mutation() {
    use math::Tensor;
    let mut tensor: Tensor<u16> = Tensor::default((2, 2, 2));
    tensor[vec![1, 1, 0]] = 111;
    tensor[1] = 222;
    println!("Should be 111 and is {:?}", tensor[vec![1, 1, 0]]);
    println!("Should be 222 and is {:?}", tensor[1]);
}

fn simple_graph() {

    /*
        the setup is quite verbose - 

        it's ok, it is not intended to be an api, but as a framework.
        later, an api for the graph will be provided.

        for now, we try to keep it as simple as possible, with enough clarity and
        ease of data access to work with.

        in addition, we try to leverage ownership mechanisms to keep the data safe from smurfs;
        (try changing stuff; keeping the graph in variable and remodify, etc...)
    */

    use math::Tensor;
    use computational_graph::{GraphBuilder, Edge};
    use operations::Operation::{Add, Dot, LeakyRelu};
    use computational_graph::Node::{
        InputNode as Input,
        ParameterNode as Parameter,
        OperationNode as Operation
    };

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
        // Set the input
        .with_node(Input(input_node_id, input))
        // Kernels
        .with_node(Parameter(weights_node_id, weights))
        .with_node(Parameter(bias_node_id, bias))
        // Determine the operations
        .with_node(Operation(dot_node_id, Dot))
        .with_node(Operation(add_node_id, Add))
        .with_node(Operation(leaky_node_id, LeakyRelu(0.57)))
        // Set the data flow
        .with_edge(Edge::new(0, input_node_id, dot_node_id))
        .with_edge(Edge::new(1, weights_node_id, dot_node_id))
        .with_edge(Edge::new(2, dot_node_id, add_node_id))
        .with_edge(Edge::new(3, bias_node_id, add_node_id))
        .with_edge(Edge::new(4, add_node_id, leaky_node_id))
        // Bam
        .build(0);

    // just print the graph, looks kinda nice with #
    println!("{:#?}", graph);
}

fn main() {
    //tensor_into_iterable();
    //tensor_mutation();
    simple_graph();
}
