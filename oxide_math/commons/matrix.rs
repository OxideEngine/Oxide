pub trait Fill {
    fn fill(number: f32) -> Self;
}

pub trait SetDiagonal {
    fn set_diagonal(&self, number: f32) -> Self;
}

pub trait IsSimilar {
    fn is_similar(&self, other: Self) -> bool;
}

pub trait IsSquare {
    fn is_sqaure(&self) -> bool;
}

pub trait Get<T> {
    fn row_length(&self) -> usize;
    fn col_length(&self) -> usize;
    fn get_row(&self, index: usize) -> T;
    fn get_col(&self, index: usize) -> T;
}

pub trait Transpose {
    fn transpose(&self) -> Self;
}

pub trait Invert {
    fn inverse(&self) -> Self;
}

pub trait Sum {
    fn sum(&self) -> f32;
}

pub trait Average {
    fn average(&self) -> f32;
}

pub trait Min {
    fn min(&self) -> f32;
    fn abs_min(&self) -> f32;
}

pub trait Max {
    fn max(&self) -> f32;
    fn abs_max(&self) -> f32;
}

pub trait Trace {
    fn trace(&self) -> f32;
}

pub trait Determinant {
    fn determinant(&self) -> f32;
}

pub trait Diagonal {
    fn diagonal(&self) -> Self;
}

pub trait OffDiagonal {
    fn off_diagonal(&self) -> Self;
}

pub trait LowerTriangle {
    fn strictly_lower_triangle(&self) -> Self;
    fn lower_triangle(&self) -> Self;
}

pub trait UpperTriangle {
    fn strictly_upper_triangle(&self) -> Self;
    fn upper_triangle(&self) -> Self;
}

pub trait Transposed {
    fn transposed(&self) -> Self;
}

pub trait Inverse {
    fn inverse(&self) -> Self;
}

pub trait MatrixHelper {
    fn make_zero() -> Self;
    fn make_identity() -> Self;
    fn make_scale() -> Self;
    fn make_rotation(theta: f32) -> Self;
    fn make_translation() -> Self;
}

pub trait Add {
    fn add(self, other: Self) -> Self;
}

pub trait Sub {
    fn sub(self, other: Self) -> Self;
}

pub trait Mul {
    fn mul(self, other: Self) -> Self;
}

pub trait Div {
    fn div(self, other: Self) -> Self;
}
