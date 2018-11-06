mod math;
mod computational_graph;

use std::vec::*;
use std::iter::FromIterator;
use math::{Tensor};

fn tensor_into_iterable() {
    let tensor: Tensor<u16> = Tensor::default((3, 3, 3));
    for item in &tensor {
        println!("tensor[{:?}] == {:?}", item.index, item.value);
    }
}

fn tensor_mutation() {
    let mut tensor: Tensor<u16> = Tensor::default((2, 2, 2));
    tensor[vec![1, 1, 0]] = 111;
    tensor[1] = 222;
    println!("Should be 111 and is {:?}", tensor[vec![1, 1, 0]]);
    println!("Should be 222 and is {:?}", tensor[1]);
}

fn main() {
    tensor_into_iterable();
    tensor_mutation();
}
