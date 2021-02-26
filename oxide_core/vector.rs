use crate::matrix;

pub trait Length {
    fn get_length(&self) -> i32;
}

pub trait Rotate {
    fn rotate(&self, rotationMatrix: matrix::Matrix) -> Self;
}

pub trait InnerProduct {
    fn inner_product(&self, _rhs: Self) -> i32;
}
