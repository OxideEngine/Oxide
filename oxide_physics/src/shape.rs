use crate::aabb::AABB;
use oxide_math::commons::vector3::Vector3;

pub trait Shape {
    fn bounding_volume(&self) -> AABB;
    fn local_bounding_volume(&self, tv: Vector3) -> AABB;
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Ball {
    radius: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
// x, y, and z are the half-extent of the cuboid
pub struct Cuboid {
    x: f32,
    y: f32,
    z: f32,
}

impl Ball {
    pub fn new(radius: f32) -> Result<Self, &'static str> {
        if radius <= 0.0 {
            Err("A ball radius must be positive.")
        } else {
            Ok(Ball { radius })
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Cuboid {
    // x, y, and z are the half-extent of the cuboid
    pub fn new(position: Vector3) -> Result<Self, &'static str> {
        if position.x == 0.0 || position.y == 0.0 || position.z == 0.0 {
            Err("Cuboid's edge must be bigger than 0")
        } else {
            Ok(Cuboid {
                x: position.x.abs(),
                y: position.y.abs(),
                z: position.z.abs(),
            })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball() {
        let ball = Ball::new(4.0).unwrap();
        assert_eq!(Ball { radius: 4.0 }, ball);
        assert_eq!(4.0, ball.radius());
    }

    #[test]
    fn test_cuboid() {
        let cuboid = Cuboid::new(Vector3 {
            x: 2.0,
            y: -4.0,
            z: 6.0,
        }).unwrap();
        assert_eq!(Cuboid {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        }, cuboid);
        assert_eq!(Vector3 { x: -2.0, y: -4.0, z: -6.0}, cuboid.mins());
        assert_eq!(Vector3 { x: 2.0, y: 4.0, z: 6.0}, cuboid.maxs());
    }
}
