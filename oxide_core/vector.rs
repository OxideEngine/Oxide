use crate::matrix;

pub trait Add {
    fn add(&self, _rhs: Self) -> Self;
}

pub trait Length<T> {
    fn get_squared_length(&self) -> T;
    fn get_length(&self) -> T;
}

pub trait Normalized {
    fn normalized(&self) -> Self;
}

pub trait Rotate {
    // TODO: Can be implemented only after matrix implemented
    fn rotate(&self, rotation_matrix: matrix::Matrix) -> Self;
}

pub trait InnerProduct<T> {
    fn inner_product(&self, _rhs: Self) -> T;
}

pub trait OuterProduct<T> {
    fn outer_product(&self, _rhs: Self) -> Self;
}

pub trait Scale<T> {
    fn scale(&self, multiplier: T) -> Self;
}

pub trait Negate {
    fn neg(&self) -> Self;
}
