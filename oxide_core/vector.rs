use crate::matrix;

pub trait LengthComputable {
    fn get_length(&self) -> i32;
}

pub trait Rotatable {
    fn rotate(&self, rotationMatrix: matrix::Matrix) -> Self;
}
