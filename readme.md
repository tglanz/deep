# deep_core

# Open Issues
- [x] design and api; operand implementation as a constant value, global to it's module
- [x] self references; problem description by example at playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=08ef7319c1c1b65800d655c2d6176ca4
    - not possible as firstly desired. implemented using indices (copies)

# Run Examples
See which {some-example} exists in the ~/examples directory (file names) and the run  
```cargo run --example {some-example}```

# Next Up - TODO: move to roadmap
## __Approximately in the order of desired implementation__
- [x] Graph traversal and visitation patterns
    - no need to get into graph theory, just iterate nodes and stuff - don't overdo
    - __the first implemented approach__ is directly iterating a given iterator. we can create specific ones, or use implemented ones (See graph::traversal)
    - __the second implemented approach__ is by using a traverser object with a visitor object. This is more usefuly when we need to keep a state during the traversal, or traverse multiple times with some continuation state. (See examples or the showcases below)
- [ ] Forward compute engine
    - [x] tensor arithmetic
    - design
    - implement first draft using a cpu backend on a simple graph (dot -> add -> leaky)
- [ ] Backward compute engine
    - how to calculate gradients (per op, per backend etc..?)
    - design
        - kinda like the forward, but keep in mind that this engine can change the Parameter Nodes
    - implement first draft using a cpu backend on __a graph__ that the forward compute engine can resolve
    
- [ ] A simple training using a graph that both forward and the backward engines can resolve
    - feed forward
    - back propogation
    - 2 different gradient decent optimizers
        - doesn't really important which, but implement at least 2 different so it enforces
          us to decouple the implementation
        - sgd and batch maybe?

- [ ] LeNet implementation and training
    - Operations implementations (Conv pool etc.. as required)
    - Training
    - Check that we get resonable result on mnist data set
    
## __Nice to have__
- [ ] Unit tests
- [ ] Docs
- [x] Graph serialization
    - to/from string
    - to/from binary

# Graph Serialization
use serde for graph serialization; can serialize and deserialize. use files etc..
see simple graph [here](graph.json)

# Showcases

## Simple graph construction
The code below demonstrate how to build a simple computation graph;  
The computation the graph represents is the composition of dot, add, leaky.
```
    // The prelude will use everything we will need
    use deep::prelude::*;

    // Some tensors
    let input_descriptor = TensorDescriptor::create(0, (3, ));
    let weights_descriptor = TensorDescriptor::create(1, (3, 3));
    let bias_descriptor = TensorDescriptor::create(2, (3, ));

    // Some Ids 
    let input_node_id = 0;
    let weights_node_id = 1;
    let bias_node_id = 2;

    let dot_node_id = 1;
    let add_node_id = 2;
    let leaky_node_id = 3;

    /*
        the computation this graph does is still the same
        leaky(matrix*vector + bias)
    */
    GraphBuilder::create()
        .with_input(input_node_id, &input_descriptor)
        .with_parameter(weights_node_id, &weights_descriptor)
        .with_parameter(bias_node_id, &bias_descriptor)
        .with_operation(dot_node_id, dot::operation())
        .with_operation(add_node_id, add::operation())
        .with_operation(leaky_node_id, leaky_relu::operation(0.57))
        .with_operand(0, input_node_id, dot_node_id, dot::operands::INPUT)
        .with_operand(1, weights_node_id, dot_node_id, dot::operands::WEIGHTS)
        .with_operand(2, dot_node_id, add_node_id, add::operands::INPUT)
        .with_operand(3, bias_node_id, add_node_id, add::operands::INPUT)
        .with_operand(4, add_node_id, leaky_node_id, leaky_relu::operands::INPUT)
        .build(0)
```

## Visiting, Traversing And Iterating
The code below is an example of how to use the visitor pattern on a given graph;  
We will implement a visitor that counts the number of nodes that we traverse on.
In this specific example; the iteration showcase is much simpler, but you get the idea...

```
    // The prelude will use everything we will need
    use deep::prelude::*;

    // Below are the two patterns we use
    // - We don't consume the graph; Owners can keep work on it
    // - We don't mutate the graph; Other borrows can take place

    fn showcase_visitation(graph: &Graph) {
        let traverser = Traverser::from_graph(&graph);
        let mut visitor = AVisitorThatCountNodes::create();
        let mut iterator = UnorderedIterator::from_graph(&graph);
        travser.traverse(&mut visitor, &mut iterator);
        println!("Counted {} nodes", visitor.get_nodes_count());
    }

    fn showcase_iteration(graph: &Graph){
        let mut nodes_count = 0;
        for node in UnorderedIterator::from_graph(&graph) {
            nodes_count += 1;
        }
        
        println!("Counted {} nodes", nodes_count);
    }

    fn main() {
        let graph = ... // Assuming we have a graph
        showcase_iteration(&graph);
        showcase_visitation(&graph);
    }

    // Below is the visitor implementation

    struct AVisitorThatCountNodes {
        nodes_count: usize,
    }

    impl AVisitorThatCountNodes {
        pub fn create() -> Self {
            AVisitorThatCountNodes {
                nodes_count: 0
            }
        }
        
        pub fn get_nodes_count(&self) -> usize {
            self.nodes_count
        }
    }

    impl Visitor for AVisitorThatCountNodes {
        fn before_visitations(&mut self) {
            // Initialize the internal state
            self.nodes_count = 0;
        }

        fn visit_node(&mut self, node: &Node, input_edges: &[&Edge], output_edges: &[&Edge]) {
            // Increment; we visited a new node
            // Note: In general, there is no gaurantee that each node will be visited only once.
            //       Nor the order is a gaurantee. Everything depends on what you trying to achieve.
            self.nodes_count += 1;
        }

        fn after_visitations(&mut self) {
            // We don't have much to do here...
        }
    }
```