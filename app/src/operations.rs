use math::{Shape};

#[derive(Debug)]
pub enum Operation {
    Add,
    Dot,
    LeakyRelu(f32),
}