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

pub type TensorsMap<'a> = HashMap<TensorId, &'a mut Tensor<u16>>;

pub trait ForwardComputer<'a> {
    fn initialize(&self) -> &'a mut Visitor;
    fn compute(&mut self, tensors_maps: TensorsMap);
    fn collect(&self) -> TensorsMap;
}