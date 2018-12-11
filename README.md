# deep

# Open Issues
- Problem description by example at playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=08ef7319c1c1b65800d655c2d6176ca4

# Graph for example
```
Graph {
    id: 0,
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
        Edge {
            id: 0,
            from_node: 0,
            to_node: 1
        },
        Edge {
            id: 1,
            from_node: 1,
            to_node: 1
        },
        Edge {
            id: 2,
            from_node: 1,
            to_node: 2
        },
        Edge {
            id: 3,
            from_node: 2,
            to_node: 2
        },
        Edge {
            id: 4,
            from_node: 2,
            to_node: 3
        }
    ]
}
```
