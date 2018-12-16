extern crate deep_core as core;

use core::{
    graph::{
        Graph,
        GraphBuilder,
        TensorDescriptor,
        node::{Node},
        edge::{Edge},
        traversal::{
            Traverser,
            Visitor,
            UnorderedIterator,
        },
    },
    operations::{
        add,
        dot,
        leaky_relu
    },
};

struct AVisitorThatPrintsStuff {
    nodes_count: usize
}

impl AVisitorThatPrintsStuff {
    fn create() -> Self {
        Self {
            nodes_count: 12312321
        }
    }
}

impl Visitor for AVisitorThatPrintsStuff {

    fn before_visitations(&mut self) {
        println!("Visitor invoked with before_visitations");
        println!("\t- initialize nodes_count to 0; just to examplify how internal states can work");
        self.nodes_count = 0;
    }

    fn visit_node(&mut self, node: &Node, input_edges: &[&Edge], output_edges: &[&Edge]) {
        println!("Visitor invoked with visit_node");
        println!("\t- node id: {}", node.get_id());
        println!("\t- input_edges");
        input_edges.iter().for_each(|edge| println!("\t\t- edge id: {}", edge.get_id()));
        println!("\t- output edges");
        output_edges.iter().for_each(|edge| println!("\t\t- edge id: {}", edge.get_id()));

        self.nodes_count += 1;
    }

    fn after_visitations(&mut self) {
        println!("Visitor invoked with after_visitations");
        println!("\t- counted {} nodes", self.nodes_count)
    }
}

fn display_the_usage_of_the_traversal_patterns(graph: &Graph) {
    let traverser = Traverser::from_graph(&graph);
    let mut visitor = AVisitorThatPrintsStuff::create();
    let mut iterator = UnorderedIterator::from_graph(&graph);
    traverser.traverse(&mut visitor, &mut iterator);
}

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

    display_the_usage_of_the_traversal_patterns(&graph);
}