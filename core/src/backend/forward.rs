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

pub trait ForwardComputer<'a> {

    type Initializer: Visitor<'a>;

    fn create_initializer(&mut self) -> Self::Initializer;
    fn compute(&mut self, initializer: Self::Initializer, inputs: HashMap<TensorId, Tensor<u16>>);
    fn collect(&self) -> HashMap<TensorId, Tensor<u16>>;
}