use crate::matrix;
use crate::vector;
use crate::traits::Float;
use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector2<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
}

impl<T> vector::Add for Vector2<T>
where
    T: Float,
{
    fn add(&self, _rhs: Self) -> Self {
        Vector2 { x: self.x + _rhs.x, y: self.y + _rhs.y}
    }
}

impl<T> vector::Length for Vector2<T> {
    fn get_squared_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }
    fn get_length(&self) -> T {
        self.get_squared_length().sqrt()
    }
}

impl<T> vector::Rotate for Vector2<T> {
    fn rotate(&self, rotation_matrix: matrix::Matrix) -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

impl<T> vector::InnerProduct for Vector2<T> {
    fn inner_product(&self, _rhs: Vector2) -> T {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

impl<T> vector::Scale for Vector2<T> {
    fn scale(&self, multiplier: T) -> Vector2 {
        Vector2 {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
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
