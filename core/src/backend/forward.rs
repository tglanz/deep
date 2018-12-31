use std::collections::{HashMap};

use super::super::{
    Tensor,
    graph::{
        TensorId,
        traversal::{
            Visitor
        },
    },
};

pub trait ForwardComputer {

    type Initializer: Visitor;

    fn initializer<'a>(&'a mut self) -> &'a mut Self::Initializer;
    fn compute<'a>(&'a mut self, inputs: &'a HashMap<TensorId, Tensor<u16>>);
    fn collect<'a>(&'a self) -> HashMap<TensorId, Tensor<u16>>;
}