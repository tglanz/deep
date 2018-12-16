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
- Graph traversal and visitation patterns
    - no need to get into graph theory, just iterate nodes and stuff - don't overdo
- Forward compute engine
    - tensor arithmetic
    - design
    - implement first draft using a cpu backend on a simple graph (dot -> add -> leaky)
- Backward compute engine
    - how to calculate gradients (per op, per backend etc..?)
    - design
        - kinda like the forward, but keep in mind that this engine can change the Parameter Nodes
    - implement first draft using a cpu backend on __a graph__ that the forward compute engine can resolve
    
- A simple training using a graph that both forward and the backward engines can resolve
    - feed forward
    - back propogation
    - 2 different gradient decent optimizers
        - doesn't really important which, but implement at least 2 different so it enforces
          us to decouple the implementation
        - sgd and batch maybe?

- LeNet implementation and training
    - Operations implementations (Conv pool etc.. as required)
    - Training
    - Check that we get resonable result on mnist data set
    
## __Nice to have__
- Unit tests
- Docs
- Graph serialization
    - to/from string
    - to/from binary

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
