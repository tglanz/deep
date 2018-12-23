use std::iter::{Iterator};

use super::{Tensor};
use super::indexers::{Indexer};

// -- Patterns

pub trait IterationPatterns<'a, T> {
    fn iter(self) -> Iter<'a, T>;
}

impl<'a, T> IterationPatterns<'a, T> for &'a Tensor<T> {
    fn iter(self) -> Iter<'a, T> {
        Iter::from(self)
    }
}

// -- Iter

pub struct Iter<'a, T> {
    tensor: &'a Tensor<T>,
    next_index: Indexer
}

impl<'a, T> Iter<'a, T> {
    fn from(tensor: &'a Tensor<T>) -> Self {
        Iter { tensor, next_index: 0 }
    }
}

#[derive(Debug)]
pub struct IterItem<'a, T> {
    value: &'a T,
    index: Indexer,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = IterItem<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index < self.tensor.shape.size() {
            let value = &self.tensor[self.next_index];
            let index = self.next_index;
            self.next_index += 1;
            Some(IterItem { value, index })
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a Tensor<T> {
    type Item = IterItem<'a, T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self)
    }
}