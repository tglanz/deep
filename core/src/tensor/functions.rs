use super::{Tensor};

pub trait Applications<T> {
    fn apply<F: Fn(&T) -> T>(&self, application: F) -> Tensor<T>;
    fn apply_mut<F: Fn(&T) -> T>(&mut self, application: F);
    fn merge<F: Fn(&T, &T) -> T>(&self, other: &Self, zipper: F) -> Tensor<T>;
    fn merge_mut<F: Fn(&T, &T) -> T>(&mut self, other: &Self, merger: F);
}

pub trait Mappings<T> {
    fn filter<F: Fn(&T) -> bool, I: Iterator<Item=T>>(&self, filterer: F) -> I;
    fn zip<Y, F: Fn(&T, &T) -> Y, I: Iterator<Item=(T, T)>>(&self, zipper: F) -> I;
    fn map<Y, F: Fn(&T) -> Y, I: Iterator<Item=T>>(&self, filterer: F) -> I;
    fn fold_left<Y, F: Fn(&Y, &T) -> Y, I: Iterator<Item=T>>(&self, accumulator: F, initial: Y) -> Y;
    fn fold_right<Y, F: Fn(&Y, &T) -> Y, I: Iterator<Item=T>>(&self, accumulator: F, initial: Y) -> Y;
}

impl<T> Applications<T> for Tensor<T> {
    fn apply<F: Fn(&T) -> T>(&self, application: F) -> Tensor<T> {
        Tensor::from_data(
            self.shape.clone(),
            self.data.iter().map(application).collect())
    }

    fn apply_mut<F: Fn(&T) -> T>(&mut self, application: F) {
        for value in self.data.iter_mut() {
            *value = application(value);
        }
    }

    fn merge<F: Fn(&T, &T) -> T>(&self, other: &Self, zipper: F) -> Tensor<T> {
        let data = self.data.iter().zip(other.data.iter()).map(|(a, b)| zipper(a, b)).collect();
        Tensor { shape: self.shape.clone(), data }
    }

    fn merge_mut<F: Fn(&T, &T) -> T>(&mut self, other: &Self, merger: F) {
        for (self_value, other_value) in self.data.iter_mut().zip(other.data.iter()) {
            *self_value = merger(self_value, other_value);
        }
    }
}

impl<T> Mappings<T> for Tensor<T> {
    fn filter<F: Fn(&T) -> bool, I: Iterator<Item=T>>(&self, _filterer: F) -> I {
        unimplemented!()
    }

    fn zip<Y, F: Fn(&T, &T) -> Y, I: Iterator<Item=(T, T)>>(&self, _zipper: F) -> I {
        unimplemented!()
    }
    
    fn map<Y, F: Fn(&T) -> Y, I: Iterator<Item=T>>(&self, _filterer: F) -> I {
        unimplemented!()
    }
    
    fn fold_left<Y, F: Fn(&Y, &T) -> Y, I: Iterator<Item=T>>(&self, _accumulator: F, _initial: Y) -> Y {
        unimplemented!()
    }
    
    fn fold_right<Y, F: Fn(&Y, &T) -> Y, I: Iterator<Item=T>>(&self, _accumulator: F, _initial: Y) -> Y {
        unimplemented!()
    }
}