#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod operations;
pub mod graph;
pub mod backend;

mod shape;
pub use shape::{Shape};

pub mod tensor;
pub use self::tensor::{Tensor};