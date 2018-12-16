extern crate deep_core as core;

use core::math::{Tensor};

fn main() {
    let mut tensor: Tensor<u16> = Tensor::default((2, 2, 2));
    tensor[vec![1, 1, 0]] = 111;
    tensor[1] = 222;
    println!("Should be 111 and is {:?}", tensor[vec![1, 1, 0]]);
    println!("Should be 222 and is {:?}", tensor[1]);
}