use super::{Tensor};
use super::super::shape::{Shape};

impl<T> Tensor<T> {
    pub fn from_data<IntoShape: Into<Shape>>(into_shape: IntoShape, data: Vec<T>) -> Self {
        Tensor { shape: into_shape.into(), data }
    }
}

impl<T: Default> Tensor<T> {
    pub fn default<IntoShape: Into<Shape>>(into_shape: IntoShape) -> Self {
        let shape = into_shape.into();
        let size = shape.size();

        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(T::default());
        }

        Tensor {
            shape, data
        }
    }
}

impl<T: Clone> Tensor<T> {
    pub fn value<IntoShape: Into<Shape>>(value: T, into_shape: IntoShape) -> Self {
        let shape = into_shape.into();
        let size = shape.size();

        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(value.clone());
        }

        Tensor {
            shape, data
        }
    }
}