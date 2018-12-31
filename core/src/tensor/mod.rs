mod classifications;
mod indexers;
mod iterators;
mod creators;
mod functions;

pub use self::classifications::*;
pub use self::indexers::*;
pub use self::iterators::*;
pub use self::creators::*;
pub use self::functions::*;

use super::shape::{Shape};

#[derive(Debug)]
pub struct Tensor<T> {
    data: Vec<T>,
    shape: Shape,
}

impl<T> Tensor<T> {
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }
}