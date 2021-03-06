extern crate deep;

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

        The graph is scrambled purposefuly to show the topological effect
    */
    GraphBuilder::create()
        .with_operand(10, bias_node_id, add_node_id, add::operands::INPUT)
        .with_parameter(weights_node_id, &weights_descriptor)
        .with_operand(9, dot_node_id, add_node_id, add::operands::INPUT)
        .with_operation(add_node_id, add::operation())
        .with_parameter(bias_node_id, &bias_descriptor)
        .with_operation(leaky_node_id, leaky_relu::operation(0.57))
        .with_operation(dot_node_id, dot::operation())
        .with_operand(11, add_node_id, leaky_node_id, leaky_relu::operands::INPUT)
        .with_operand(7, input_node_id, dot_node_id, dot::operands::INPUT)
        .with_input(input_node_id, &input_descriptor)
        .with_operand(8, weights_node_id, dot_node_id, dot::operands::WEIGHTS)
        .build(0)
}

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

impl<'a> Visitor<'a> for AVisitorThatPrintsStuff {

    fn before_visitations(&mut self) {
        println!("Visitor invoked with before_visitations");
        println!("\t- initialize nodes_count to 0; just to examplify how internal states can work");
        self.nodes_count = 0;
    }

    fn visit_node(&mut self, node: &'a Node, input_edges: &[&'a Edge], output_edges: &[&'a Edge]) {
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
    let mut iterator = TopologicalIterator::from_graph(&graph);
    traverser.traverse(&mut visitor, &mut iterator);
}

fn main() {
    let input_descriptor = TensorDescriptor::create(0, (3, ));
    let weights_descriptor = TensorDescriptor::create(1, (3, 3));
    let bias_descriptor = TensorDescriptor::create(2, (3, ));
    let graph = construct_graph(&input_descriptor, &weights_descriptor, &bias_descriptor);

    display_the_usage_of_the_traversal_patterns(&graph);
}