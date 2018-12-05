use std::ops::{Index, IndexMut};
use std::vec::*;

use math::{Shape};

pub enum Error {
    ShapeMismatch
}

pub fn position_to_index(position: &[usize], shape: &Shape) -> usize {
    let mut result = position[0];
    let mut shape_multiplier = 1;
    for component_index in 0..position.len() - 1 {
        shape_multiplier *= shape[component_index];
        result += position[component_index + 1] * shape_multiplier;
    }

    result
}

// -- Tensor

#[derive(Debug)]
pub struct Tensor<T> {
    data: Vec<T>,
    shape: Shape,
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

// Iterator

#[derive(Debug)]
pub struct TensorIterationItem<'a, T: 'a> {
    pub value: &'a T,
    pub index: usize,
}

pub struct TensorIterator<'a, T: 'a> {
    pub tensor: &'a Tensor<T>,
    pub index: usize,
}

impl<'a, T: 'a> Iterator for TensorIterator<'a, T> {
    type Item = TensorIterationItem<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tensor.shape.size() {
            let value = &self.tensor.data[self.index];
            self.index += 1;
            Some(TensorIterationItem{
                index: self.index,
                value, 
            })
        } else {
            None
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a Tensor<T> {
    type Item = TensorIterationItem<'a, T>;
    type IntoIter = TensorIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        TensorIterator { index: 0, tensor: self }
    }
}

// -- Index

impl<T> Index<usize> for Tensor<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        &mut self.data[index]
    }
}

impl<T> Index<Vec<usize>> for Tensor<T> {
    type Output = T;
    fn index(&self, position: Vec<usize>) -> &Self::Output {
        let index = position_to_index(&position, &self.shape);
        &self.data[index]
    }
}

impl<T> IndexMut<Vec<usize>> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: Vec<usize>) -> &'a mut T {
        let index = position_to_index(&position, &self.shape);
        &mut self.data[index]
    }
}