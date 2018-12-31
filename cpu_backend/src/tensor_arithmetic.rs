use core::{
    Shape,
    Tensor,
    tensor::*
};

pub enum ArithmeticError {
    RankMismatch(String),
    ShapesMismatch(Shape, Shape),
}

trait TensorArithmetic {
    fn multiply_scalar(&self, scalar: u16) -> Result<Tensor<u16>, ArithmeticError>;
    fn matrix_vector_multiplication(&self, vector: &Tensor<u16>) -> Result<Tensor<u16>, ArithmeticError>;
    fn matrix_matrix_multiplication(&self, matrix: &Tensor<u16>) -> Result<Tensor<u16>, ArithmeticError>;
}

impl TensorArithmetic for Tensor<u16> {

    fn multiply_scalar(&self, scalar: u16) -> Result<Tensor<u16>, ArithmeticError> {
        Ok(self.apply(|x| x * scalar))
    }

    fn matrix_vector_multiplication(&self, vector: &Tensor<u16>) -> Result<Tensor<u16>, ArithmeticError> {
        if self.is_matrix() == false {
            return Err(ArithmeticError::RankMismatch("matrix".to_string()));
        }

        if vector.is_vector() == false {
            return Err(ArithmeticError::RankMismatch("vector".to_string()));
        }

        let rows = self.get_shape()[0];
        let cols = self.get_shape()[1];

        if cols != vector.get_shape()[0] {
            return Err(ArithmeticError::ShapesMismatch (
                self.get_shape().clone(),
                vector.get_shape().clone()
            ));
        }

        let mut result_vector = Tensor::default((rows,));
        for row in 0..rows {
            for col in 0..cols {
                result_vector[row] += self[(row, col)] * vector[col];
            }
        }

        Ok(result_vector)
    }

    fn matrix_matrix_multiplication(&self, matrix: &Tensor<u16>) -> Result<Tensor<u16>, ArithmeticError> {
        if self.is_matrix() == false || matrix.is_matrix() == false {
            return Err(ArithmeticError::RankMismatch("matrix".to_string()));
        }

        let left_shape = self.get_shape();
        let right_shape = matrix.get_shape();

        if left_shape[1] != right_shape[0] {
            return Err(ArithmeticError::ShapesMismatch(
                left_shape.clone(),
                right_shape.clone(),
            ));
        }

        let mut result = Tensor::default((left_shape[1], right_shape[0]));
        for row in 0..result.get_shape()[0] {
            for col in 0..result.get_shape()[1] {
                for idx in 0..left_shape[1] {
                    result[(row, col)] += self[(row, idx)] * matrix[(idx, col)];
                }
            }
        }

        Ok(result)
    }
}
