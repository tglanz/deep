use super::{
    visitor::{Visitor},
    super::{
        graph::{Graph},
        node::{Node},
        edge::{Edge},
        edges_indices::{EdgesIndices},
    }
};

pub struct Traverser<'a> {
    graph: &'a Graph,
    edges_indices: EdgesIndices
}

impl<'a> Traverser<'a> {
    pub fn from_graph(graph: &'a Graph) -> Self {
        let edges_indices = EdgesIndices::from_graph(graph);
        Traverser {
            graph,
            edges_indices,
        }
    }

    pub fn traverse<TVisitor: Visitor<'a>, TIterator>(&self, visitor: &mut TVisitor, iterator: &mut TIterator)
        where
            TIterator: Iterator<Item = &'a Node> {
        
        visitor.before_visitations();

        for node in iterator {
            let graph_edges = self.graph.get_edges();

            let mut input_edges: Vec<&Edge> = self.edges_indices.get_indices_to(node.get_id())
                .unwrap_or(&Vec::new())
                .iter()
                .map(|index| &graph_edges[*index])
                .collect();
            

            let mut output_edges: Vec<&Edge> = self.edges_indices.get_indices_from(node.get_id())
                .unwrap_or(&Vec::new())
                .iter()
                .map(|index| &graph_edges[*index])
                .collect();

            visitor.visit_node(&node, &input_edges, &output_edges);
        }

        visitor.after_visitations();
    }
}