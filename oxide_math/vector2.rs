use std::ops;

use serde::{Deserialize, Serialize};

use crate::vector;
use crate::vector::*;
use crate::{matrix, vector3};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl vector::Rotate for Vector2 {
    fn rotate(&self, _rotation_matrix: matrix::Matrix) -> Self {
        panic!("TODO")
    }
}

impl vector::InnerProduct for Vector2 {
    fn inner_product(&self, _rhs: &Self) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

impl vector::OuterProduct for Vector2 {
    fn outer_product(&self, _rhs: &Self) -> vector3::Vector3 {
        let self_vec3 = vector3::Vector3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        };
        let rhs_vec3 = vector3::Vector3 {
            x: _rhs.x,
            y: _rhs.y,
            z: 0.0,
        };
        self_vec3.outer_product(&rhs_vec3)
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

impl vector::Negate for Vector2 {
    fn negate(&self) -> Self {
        self.scale(-1.0)
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

impl vector::Distance for Vector2 {
    fn distance(&self, _rhs: &Self) -> f32 {
        (self - _rhs).get_length()
    }
}

impl vector::Clamp for Vector2 {
    fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
}

impl vector::Project for Vector2 {
    fn project_on_to(&self, _rhs: &Self) -> Self {
        return _rhs
            .normalize()
            .scale(self.inner_product(_rhs) / _rhs.get_length());
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

impl<'a, 'b> ops::Sub<&'b Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn sub(self, _rhs: &'b Vector2) -> Vector2 {
        Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}
