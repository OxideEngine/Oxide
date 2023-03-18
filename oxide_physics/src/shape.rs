use crate::aabb::AABB;
use oxide_math::commons::vector3::Vector3;

pub trait Shape {
    fn bounding_volume(&self) -> AABB;
    fn local_bounding_volume(&self, tv: Vector3) -> AABB;
}

pub struct Ball {
    radius: f32,
}

// x, y, and z are the half-extent of the cuboid
pub struct Cuboid {
    x: f32,
    y: f32,
    z: f32,
}

impl Ball {
    pub fn new(radius: f32) -> Self {
        assert!(radius > 0.0, "A ball radius must be positive.");

        Ball { radius }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Cuboid {
    pub fn new(position: &Vector3) -> Self {
        Cuboid {
            x: position.x,
            y: position.y,
            z: position.z,
        }
    }

    pub fn mins(&self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn maxs(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
