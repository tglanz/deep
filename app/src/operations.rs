use math::{Shape};

#[derive(Debug)]
pub enum Operation {
    Add,
    Mul,
    Dot,
    BatchNorm2d(Shape),
    LeakyRelu(f32),
}