use crate::commons::vector;
use crate::commons::vector::*;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl vector::Length for Vector3 {
    fn get_squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn get_length(&self) -> f32 {
        self.get_squared_length().sqrt()
    }

    fn normalize(&self) -> Self {
        self.scale(self.get_length().recip())
    }
}

// impl vector::Rotate for Vector3 {
//     fn rotate(&self, _rotation_matrix: matrix::Matrix) -> Self {
//         panic!("TODO")
//     }
// }

impl vector::InnerProduct for Vector3 {
    fn inner_product(&self, _rhs: &Self) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }
}

impl vector::OuterProduct for Vector3 {
    fn outer_product(&self, _rhs: &Self) -> Vector3 {
        Vector3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x,
        }
    }
}

impl vector::Scale for Vector3 {
    fn scale(&self, multiplier: f32) -> Self {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
        }
    }
}

impl vector::Negate for Vector3 {
    fn negate(&self) -> Self {
        self.scale(-1.0)
    }
}

impl vector::Distance for Vector3 {
    fn distance(&self, _rhs: &Self) -> f32 {
        (self - _rhs).get_length()
    }
}

impl vector::Clamp for Vector3 {
    fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }
}

impl vector::Project for Vector3 {
    fn project_on_to(&self, _rhs: &Self) -> Self {
        return _rhs
            .normalize()
            .scale(self.inner_product(_rhs) / _rhs.get_length());
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: &'b Vector3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}
