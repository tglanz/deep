use std::ops::{Index, IndexMut};

use super::{Tensor};

pub type Indexer = usize;
pub type Position = Vec<Indexer>;

pub trait PositionStacking {
    fn indexer_to_position(&self, indexer: Indexer) -> Position;
    fn position_to_indexer(&self, position: &Position) -> Indexer;
}

impl<T> PositionStacking for Tensor<T> {
    fn indexer_to_position(&self, _indexer: Indexer) -> Position {
        unimplemented!()
    }

    fn position_to_indexer(&self, position: &Position) -> Indexer {
        let shape = self.get_shape();
        let mut result = position[0];
        let mut shape_multiplier = 1;
        for component_index in 0..position.len() - 1 {
            shape_multiplier *= shape[component_index];
            result += position[component_index + 1] * shape_multiplier;
        }

        result
    }
}

// Implements for usize
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

// Implements for Vec<usize>
impl<T> Index<&Vec<usize>> for Tensor<T> {
    type Output = T;
    fn index(&self, position: &Vec<usize>) -> &Self::Output {
        let index = self.position_to_indexer(position);
        &self.data[index]
    }
}

impl<T> IndexMut<&Vec<usize>> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: &Vec<usize>) -> &'a mut T {
        let index = self.position_to_indexer(position);
        &mut self.data[index]
    }
}


// Implements for &[usize]
impl<T> Index<&[usize]> for Tensor<T> {
    type Output = T;
    fn index(&self, position: &[usize]) -> &Self::Output {
        let index = self.position_to_indexer(&position.to_vec());
        &self.data[index]
    }
}

impl<T> IndexMut<&[usize]> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: &[usize]) -> &'a mut T {
        let index = self.position_to_indexer(&position.to_vec());
        &mut self.data[index]
    }
}

// Implements for (usize,)
impl<T> Index<(usize,)> for Tensor<T> {
    type Output = T;
    fn index(&self, position: (usize,)) -> &Self::Output {
        &self.data[position.0]
    }
}

impl<T> IndexMut<(usize,)> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: (usize,)) -> &'a mut T {
        &mut self.data[position.0]
    }
}

// Implements for (usize, usize)
impl<T> Index<(usize, usize)> for Tensor<T> {
    type Output = T;
    fn index(&self, position: (usize, usize)) -> &Self::Output {
        &self[&vec![position.0, position.1]]
    }
}

impl<T> IndexMut<(usize, usize)> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: (usize, usize)) -> &'a mut T {
        &mut self[&vec![position.0, position.1]]
    }
}

// Implements for (usize, usize, usize)
impl<T> Index<(usize, usize, usize)> for Tensor<T> {
    type Output = T;
    fn index(&self, position: (usize, usize, usize)) -> &Self::Output {
        &self[&vec![position.0, position.1, position.2]]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: (usize, usize, usize)) -> &'a mut T {
        &mut self[&vec![position.0, position.1, position.2]]
    }
}

// Implements for (usize, usize, usize, usize)
impl<T> Index<(usize, usize, usize, usize)> for Tensor<T> {
    type Output = T;
    fn index(&self, position: (usize, usize, usize, usize)) -> &Self::Output {
        &self[&vec![position.0, position.1, position.2, position.3]]
    }
}

impl<T> IndexMut<(usize, usize, usize, usize)> for Tensor<T> {
    fn index_mut<'a>(&'a mut self, position: (usize, usize, usize, usize)) -> &'a mut T {
        &mut self[&vec![position.0, position.1, position.2]]
    }
}