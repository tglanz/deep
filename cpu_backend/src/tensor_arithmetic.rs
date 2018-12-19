use core::{
    math::{
        Tensor, Shape
    }
};

pub enum ArithmeticError {
    RankMismatch(String),
    ShapesMismatch(Shape, Shape),
}

fn check_sizes_match<T>(left: &Tensor<T>, right: &Tensor<T>) -> Option<ArithmeticError> {
    if left.get_shape().size() == right.get_shape().size() {
        None
    } else {
        Some(ArithmeticError::ShapesMismatch(
            left.get_shape().clone(),
            right.get_shape().clone())
    }
}

trait TensorArithmetic {
    fn zip(&self, other: &Tensor<u16>, zipper: fn(&u16, &u16) -> u16) -> Result<Tensor<u16>, ArithmeticError>;
    fn zip_mut(&mut self, other: &Tensor<u16>, zipper: fn(&u16, &u16) -> u16) -> Result<(), ArithmeticError>;
    fn matrix_vector_multiplication(&self, vector: &Tensor<u16>) -> Result<Tensor<u16>, ArithmeticError>;
    fn matrix_matrix_multiplication(&self, matrix: &Tensor<u16>) -> Result<Tensor<u16>, ArithmeticError>;
}

impl TensorArithmetic for Tensor<u16> {

    fn zip(&self, other: &Tensor<u16>, zipper: fn(&u16, &u16) -> u16) -> Result<Tensor<u16>, ArithmeticError> {
        if let Some(error) = check_sizes_match(self, other) {
            return Err(error);
        }

        let mut result = Tensor::default(self.get_shape().clone());
        for idx in 0..self.get_shape().size() {
            result[idx] = zipper(&self[idx], &other[idx]);
        }

        Ok(result)
    }

    fn zip_mut(&mut self, other: &Tensor<u16>, zipper: fn(&u16, &u16) -> u16) -> Result<(), ArithmeticError> {
        if let Some(error) = check_sizes_match(self, other) {
            return Err(error);
        }

        for idx in 0..self.get_shape().size() {
            self[idx] = zipper(&self[idx], &other[idx]);
        }

        Ok(())
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
            return Err(ArithmeticError::ShapesMismatch(
                self.get_shape().clone(),
                vector.get_shape().clone());
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
            return Err(ArithmeticError::ShapesMismatch {
                left: left_shape.clone(),
                right: right_shape.clone(),
            });
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