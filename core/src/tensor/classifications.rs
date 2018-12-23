use super::{Tensor};

pub trait Classifications {
    fn is_vector(&self) -> bool;
    fn is_matrix(&self) -> bool;
}

impl<T> Classifications for Tensor<T> {
    fn is_vector(&self) -> bool {
        self.shape.rank() == 1
    }

    fn is_matrix(&self) -> bool {
        self.shape.rank() == 2
    }
}