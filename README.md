# deep

# Open Issues
- Problem description by example at playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=08ef7319c1c1b65800d655c2d6176ca4

# Run Examples
```cargo run --example {some-example}```
See /examples directory to see which examples exists.. we gathering the up

# Graph for example
```
Graph {
    id: 0,
    objects: GraphObjects {
        nodes: [
            InputNode(
                0,
                Tensor {
                    data: [
                        0,
                        0,
                        0
                    ],
                    shape: Shape {
                        dimensions: [
                            3
                        ]
                    }
                }
            ),
            ParameterNode(
                1,
                Tensor {
                    data: [
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0
                    ],
                    shape: Shape {
                        dimensions: [
                            3,
                            3
                        ]
                    }
                }
            ),
            ParameterNode(
                2,
                Tensor {
                    data: [
                        0,
                        0,
                        0
                    ],
                    shape: Shape {
                        dimensions: [
                            3
                        ]
                    }
                }
            ),
            OperationNode(
                1,
                Dot
            ),
            OperationNode(
                2,
                Add
            ),
            OperationNode(
                3,
                LeakyRelu(
                    0.57
                )
            )
        ],
        edges: [
            OperandEdge(
                0,
                (
                    0,
                    1
                ),
                0
            ),
            OperandEdge(
                1,
                (
                    1,
                    1
                ),
                1
            ),
            OperandEdge(
                2,
                (
                    1,
                    2
                ),
                0
            ),
            OperandEdge(
                3,
                (
                    2,
                    2
                ),
                0
            ),
            OperandEdge(
                4,
                (
                    2,
                    3
                ),
                0
            )
        ]
    },
    edges_indices: EdgesIndices {
        indices_by_from: {
            2: [
                3,
                4
            ],
            0: [
                0
            ],
            1: [
                1,
                2
            ]
        },
        indices_by_to: {
            3: [
                4
            ],
            2: [
                2,
                3
            ],
            1: [
                0,
                1
            ]
        }
    }
}
```
