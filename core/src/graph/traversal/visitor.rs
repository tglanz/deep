use super::super::{
    node::{Node},
    edge::{Edge},
};

pub trait Visitor {
    /// Invoked prior to all visitations. Gives a chance for construction
    fn before_visitations(&mut self);

    fn visit_node(
        &mut self,
        node: &Node,
        input_edges: &[&Edge],
        output_edges: &[&Edge]);

    /// Invoked subsequent to all visitations. Gives a chance for destruction
    fn after_visitations(&mut self);
}