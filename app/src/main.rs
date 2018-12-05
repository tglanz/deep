#![allow(dead_code)]
#![allow(unused_imports)]

mod math;
mod operations;
mod computational_graph;

use math::{Tensor};
use operations::Operation;

use computational_graph::{Graph, Node, Uniform};
use computational_graph::Edge::{NodeNodeEdge, UniformNodeEdge};

fn tensor_into_iterable() {
    let tensor: Tensor<u16> = Tensor::default((3, 3, 3));
    for item in &tensor {
        println!("tensor[{:?}] == {:?}", item.index, item.value);
    }
}

fn tensor_mutation() {
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

    let vector: Tensor<u16> = Tensor::default((3, 3));
    let vector_uniform_id = 1;

    let matrix: Tensor<u16> = Tensor::default((3,));
    let matrix_uniform_id = 2;

    let bias: Tensor<u16> = Tensor::default((3,));
    let bias_uniform_id = 3;

    let dot_node_id = 1;
    let add_node_id = 2;
    let leaky_node_id = 3;

    /*
        the computation this graph does is
        leaky(matrix*vector + bias)
     */

    let graph = Graph::new(0)
        // define the entries for the graph
        .with_uniform(Uniform::new(matrix_uniform_id, matrix))
        .with_uniform(Uniform::new(vector_uniform_id, vector))
        .with_uniform(Uniform::new(bias_uniform_id, bias))
        // define a node for dot operation
        .with_node(Node::new(dot_node_id, Operation::Dot))
        // define the operands into the dot node, notice that both originate from a uniform
        .with_edge(UniformNodeEdge(1, (matrix_uniform_id, dot_node_id)))
        .with_edge(UniformNodeEdge(2, (vector_uniform_id, dot_node_id)))
        // define a node for add operation
        .with_node(Node::new(add_node_id, Operation::Add))
        // define the operands into the addd node, notice that one originate from the result from
        // the dot operation node, the other from a uniform
        .with_edge(NodeNodeEdge(3, (dot_node_id, add_node_id)))
        .with_edge(UniformNodeEdge(4, (bias_uniform_id, add_node_id)))
        // define a node for leaky operation
        .with_node(Node::new(leaky_node_id, Operation::LeakyRelu(0.57)))
        // define the (single) operand to the leaky node
        .with_edge(NodeNodeEdge(5, (add_node_id, leaky_node_id)));

    // just print the graph, it doesn't look pretty
    println!("Graph created: {:?}", graph);
}

fn main() {
    //tensor_into_iterable();
    //tensor_mutation();
    simple_graph();
}
