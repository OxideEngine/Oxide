use crate::matrix;
use crate::vector;
use crate::vector::*;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl vector::Length for Vector4 {
    fn get_squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn get_length(&self) -> f32 {
        self.get_squared_length().sqrt()
    }

    fn normalize(&self) -> Self {
        self.scale(self.get_length().recip())
    }
}

impl vector::Rotate for Vector4 {
    fn rotate(&self, _rotation_matrix: matrix::Matrix) -> Self {
        panic!("TODO")
    }
}

impl vector::InnerProduct for Vector4 {
    fn inner_product(&self, _rhs: &Self) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z + self.w * _rhs.w
    }
}

impl vector::Scale for Vector4 {
    fn scale(&self, multiplier: f32) -> Self {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
            w: self.w * multiplier,
        }
    }

    fn negate(&self) -> Self {
        self.scale(-1.0)
    }
}

impl ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, _rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, _rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}
