use super::super::{
    node::{Node},
    edge::{Edge},
};

pub trait Visitor<'a> {
    /// Invoked prior to all visitations. Gives a chance for construction
    fn before_visitations(&mut self);

    fn visit_node(
        &mut self,
        node: &'a Node,
        input_edges: &[&'a Edge],
        output_edges: &[&'a Edge]);

    /// Invoked subsequent to all visitations. Gives a chance for destruction
    fn after_visitations(&mut self);
}