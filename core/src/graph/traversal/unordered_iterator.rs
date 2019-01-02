use super::super::{
    graph::{Graph},
    node::{Node, Nodes},
};

pub struct UnorderedIterator<'a> {
    nodes: &'a Nodes,
    cursor: usize,
}

impl<'a> UnorderedIterator<'a> {
    pub fn from_graph(graph: &'a Graph) -> Self {
        Self {
            nodes: graph.get_nodes(),
            cursor: 0
        }
    }
}

impl<'a> Iterator for UnorderedIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.nodes.get(self.cursor);
        self.cursor += 1;
        println!("cursor {:#?}", self.cursor);
        item
    }
}