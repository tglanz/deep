use super::super::{
    graph::Graph,
    virtual_graph::VirtualGraph,
    node::Node,
};

pub struct TopologicalIterator<'a> {
    virtual_graph: VirtualGraph<'a>,
    orphan_nodes_indices: Vec<usize>,
}

impl<'a> TopologicalIterator<'a> {
    pub fn from_graph(graph: &'a Graph) -> Self {
        let virtual_graph = VirtualGraph::from_graph(graph);
        let orphan_nodes_indices = virtual_graph.get_orphan_nodes_indices();

        Self {
            virtual_graph,
            orphan_nodes_indices,
        }
    }
}

impl<'a> Iterator for TopologicalIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(orphan_node_index) = self.orphan_nodes_indices.pop() {
            let orphan_node: &'a Node = *(self.virtual_graph.get_node(orphan_node_index).unwrap());

            for edge_from_orphan_index in self.virtual_graph.get_indices_of_edges_from(orphan_node_index) {
                if let Some(edge) = self.virtual_graph.get_edge(edge_from_orphan_index) {
                    let child_node_id = edge.get_connection().1;
                    if let Some(child_index) = self.virtual_graph.get_index_of_source_node(&child_node_id) {
                        if self.virtual_graph.get_indices_of_edges_into(*child_index).len() == 1 {
                            // This is the last edge into child_node
                            self.orphan_nodes_indices.push(*child_index);
                        }
                    }
                }

                self.virtual_graph.remove_edge(edge_from_orphan_index);
            }

            Some(orphan_node)
        } else {
            None
        }
    }
}