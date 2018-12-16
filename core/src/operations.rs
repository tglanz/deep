pub type Operand = u8;

#[derive(Debug)]
pub enum Operation {
    Add,
    Dot,
    LeakyRelu(f32),
}

pub mod add {
    use super::*;
    pub fn operation() -> Operation { Operation::Add }
    pub mod operands {
        pub fn input() -> super::Operand { 0 }
    }
}

pub mod dot {
    use super::*;
    pub fn operation() -> Operation { Operation::Dot }
    pub mod operands {
        pub fn input() -> super::Operand { 0 }
        pub fn weights() -> super::Operand { 1 }
    }
}

pub mod leaky_relu {
    use super::*;
    pub fn operation(param: f32) -> Operation { Operation::LeakyRelu(param) }
    pub mod operands {
        pub fn input() -> super::Operand { 0 }
    }
}