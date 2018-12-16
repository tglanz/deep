use std::ops::{Index};

#[derive(Clone, Debug)]
pub struct Shape {
    dimensions: Vec<usize>
}

impl Shape {
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    pub fn size(&self) -> usize {
        self.dimensions.iter().fold(1, |acc, x| acc * x)
    }
}


impl From<Vec<usize>> for Shape {
    fn from(dimensions: Vec<usize>) -> Self {
        Self {
            dimensions
        }
    }
}

impl From<usize> for Shape {
    fn from(dimension: usize) -> Self {
        Self {
            dimensions: vec![dimension]
        }
    }
}

impl From<(usize,)> for Shape {
    fn from(dimensions: (usize,)) -> Self {
        Self {
            dimensions: vec![dimensions.0]
        }
    }
}

impl From<(usize, usize)> for Shape {
    fn from(dimensions: (usize, usize)) -> Self {
        Self {
            dimensions: vec![dimensions.0, dimensions.1]
        }
    }
}

impl From<(usize, usize, usize)> for Shape {
    fn from(dimensions: (usize, usize, usize)) -> Self {
        Self {
            dimensions: vec![dimensions.0, dimensions.1, dimensions.2]
        }
    }
}

impl From<(usize, usize, usize, usize)> for Shape {
    fn from(dimensions: (usize, usize, usize, usize)) -> Self {
        Self {
            dimensions: vec![dimensions.0, dimensions.1, dimensions.2, dimensions.3]
        }
    }
}

// -- Index
impl Index<usize> for Shape {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.dimensions[index]
    }
}