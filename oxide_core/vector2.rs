use crate::matrix;
use crate::vector;

pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl vector::LengthComputable for Vector2 {
    fn get_length(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl vector::Rotatable for Vector2 {
    fn rotate(&self, rotationMatrix: matrix::Matrix) -> Self {
        Vector2 { x: 0, y: 0 }
    }
}
