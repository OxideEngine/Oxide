use crate::matrix;
use crate::vector;

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {}

impl vector::Length for Vector2 {
    fn get_squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    fn get_length(&self) -> f32 {
        self.get_squared_length().sqrt()
    }
}

impl vector::Rotate for Vector2 {
    fn rotate(&self, rotation_matrix: matrix::Matrix) -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

impl vector::InnerProduct for Vector2 {
    fn inner_product(&self, _rhs: Vector2) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

impl vector::Scale for Vector2 {
    fn scale(&self, multiplier: f32) -> Vector2 {
        Vector2 {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }
}
