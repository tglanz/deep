use math::{Tensor};

pub type UniformId = u16;
pub type Uniforms = Vec<Uniform>;

#[derive(Debug)]
pub struct Uniform {
    id: UniformId,
    tensor: Tensor<u16>,
}

impl Uniform {
    pub fn new(id: UniformId, tensor: Tensor<u16>) -> Self {
        Uniform {
            id, tensor
        }
    }
}