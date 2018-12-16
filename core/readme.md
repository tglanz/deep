# deep-core

# Open Issues
- [ ] design and api; operand implementation as a constant value, global to it's module
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
    - tensor arithmetic
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
- Unit tests
- Docs
- Graph serialization
    - to/from string
    - to/from binary

# Showcases

## Simple graph construction
The code below demonstrate how to build a simple computation graph;  
The computation the graph represents is the composition of dot, add, leaky.
```
// The objects we'll need here
use core::{
    graph::{GraphBuilder, TensorDescriptor},
    operations::{
        add,
        dot,
        leaky_relu
    },
};

// Initialize the id values (*_id vars below)

let graph = GraphBuilder::create()
    .with_input(input_node_id, TensorDescriptor::create(input_tensor_id).with_shape((3, )))
    .with_parameter(weights_node_id, TensorDescriptor::create(weights_tensor_id).with_shape((3, 3)))
    .with_parameter(bias_node_id, TensorDescriptor::create(bias_tensor_id).with_shape((3, )))
    .with_operation(dot_node_id, dot::operation())
    .with_operation(add_node_id, add::operation())
    .with_operation(leaky_node_id, leaky_relu::operation(lrelu_alpha_param))
    .with_operand(operand_0_id, input_node_id, dot_node_id, dot::operands::input())
    .with_operand(operand_1_id, weights_node_id, dot_node_id, dot::operands::weights())
    .with_operand(operand_2_id, dot_node_id, add_node_id, add::operands::input())
    .with_operand(operand_3_id, bias_node_id, add_node_id, add::operands::input())
    .with_operand(operand_4_id, add_node_id, leaky_node_id, leaky_relu::operands::input())
    .build(graph_id);
```

## Visiting, Traversing And Iterating
The code below is an example of how to use the visitor pattern on a given graph;  
We will implement a visitor that counts the number of nodes that we traverse on.
In this specific example; the iteration showcase is much simpler, but you get the idea...

```
// The objects we'll need here
use core::{
    graph::{
        traversal::{
            Traverser,
            Visitor,
            UnorderedIterator,
        },
    },
};

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

# Graph for example
The serialization below is made by the default Debug rust formatter, the format is not what important here and it is subject to change in the future. The which, and what data, is the important thing here.

```
Graph {
    id: 0,
    nodes: [
        InputNode {
            id: 0,
            tensor_descriptor: TensorDescriptor {
                tensor_id: 0,
                shape: Some(
                    Shape {
                        dimensions: [
                            3
                        ]
                    }
                )
            }
        },
        ParameterNode {
            id: 1,
            tensor_descriptor: TensorDescriptor {
                tensor_id: 1,
                shape: Some(
                    Shape {
                        dimensions: [
                            3,
                            3
                        ]
                    }
                )
            }
        },
        ParameterNode {
            id: 2,
            tensor_descriptor: TensorDescriptor {
                tensor_id: 2,
                shape: Some(
                    Shape {
                        dimensions: [
                            3
                        ]
                    }
                )
            }
        },
        OperationNode {
            id: 1,
            operation: Dot
        },
        OperationNode {
            id: 2,
            operation: Add
        },
        OperationNode {
            id: 3,
            operation: LeakyRelu(
                0.57
            )
        }
    ],
    edges: [
        OperandEdge {
            id: 0,
            connection: (
                0,
                1
            ),
            operand: 0
        },
        OperandEdge {
            id: 1,
            connection: (
                1,
                1
            ),
            operand: 1
        },
        OperandEdge {
            id: 2,
            connection: (
                1,
                2
            ),
            operand: 0
        },
        OperandEdge {
            id: 3,
            connection: (
                2,
                2
            ),
            operand: 0
        },
        OperandEdge {
            id: 4,
            connection: (
                2,
                3
            ),
            operand: 0
        }
    ]
}
```
