use crate::matrix;
use crate::vector;
use crate::vector::*;
use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {}

impl vector::Rotate for Vector2 {
    fn rotate(&self, _rotation_matrix: matrix::Matrix) -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl vector::InnerProduct for Vector2 {
    fn inner_product(&self, _rhs: &Self) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

impl vector::Scale for Vector2 {
    fn scale(&self, multiplier: f32) -> Self {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }
}

impl vector::Length for Vector2 {
    fn get_squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn get_length(&self) -> f32 {
        self.get_squared_length().sqrt()
    }

    fn normalize(&self) -> Self {
        self.scale(self.get_length().recip())
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}
