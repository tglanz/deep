extern crate deep_core as core;

use core::math::Tensor;

fn main() {
    let tensor: Tensor<u16> = Tensor::default((3, 3, 3));
    for item in &tensor {
        println!("tensor[{:?}] == {:?}", item.index, item.value);
    }
}