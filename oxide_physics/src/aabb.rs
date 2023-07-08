use crate::collide_broad_phase::{BoundingVolume, HasBoundingVolume};
use oxide_math::commons::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct AABB {
    pub mins: Vector3,
    pub maxs: Vector3,
}

pub fn aabb<S>(shape: &S, tv: Vector3) -> AABB
where
    S: HasBoundingVolume<AABB>,
{
    shape.bounding_volume(tv)
}

pub fn local_aabb<S>(shape: &S) -> AABB
where
    S: HasBoundingVolume<AABB>,
{
    shape.local_bounding_volume()
}

impl AABB {
    pub fn new(mins: Vector3, maxs: Vector3) -> AABB {
        AABB { mins, maxs }
    }

    pub fn mins(&self) -> Vector3 {
        Vector3 {
            x: self.mins.x,
            y: self.mins.y,
            z: self.mins.z,
        }
    }

    pub fn maxs(&self) -> Vector3 {
        Vector3 {
            x: self.maxs.x,
            y: self.maxs.y,
            z: self.maxs.z,
        }
    }
}

impl BoundingVolume for AABB {
    // check if the bounding volume 'bv' intersects with self
    fn intersects(&self, other: &AABB) -> bool {
        self.mins.x <= other.maxs.x
            && self.mins.y <= other.maxs.y
            && self.mins.z <= other.maxs.z
            && self.maxs.x >= other.mins.x
            && self.maxs.y >= other.mins.y
            && self.maxs.z >= other.mins.z
    }

    // check if self contains the 'bv'
    fn contains(&self, other: &AABB) -> bool {
        self.mins.x <= other.mins.x
            && self.mins.y <= other.mins.y
            && self.mins.z <= other.mins.z
            && self.maxs.x >= other.maxs.x
            && self.maxs.y >= other.maxs.y
            && self.maxs.z >= other.maxs.z
    }

    // merge this bounding volume with the other 'bv'
    fn merged(&self, other: &AABB) -> AABB {
        AABB {
            mins: Vector3 {
                x: self.mins.x.min(other.mins.x),
                y: self.mins.y.min(other.mins.y),
                z: self.mins.z.min(other.mins.z),
            },
            maxs: Vector3 {
                x: self.maxs.x.max(other.maxs.x),
                y: self.maxs.y.max(other.maxs.y),
                z: self.maxs.z.max(other.maxs.z),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::Ball;

    #[test]
    fn test_intersects() {
        let ball = Ball::new(1.0).unwrap();
        let ball_aabb0 = ball.local_bounding_volume();
        let ball_aabb1 = ball.bounding_volume(Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        });
        let ball_aabb2 = ball.bounding_volume(Vector3 {
            x: 1.5,
            y: 1.5,
            z: 1.5,
        });
        let ball_aabb3 = ball.bounding_volume(Vector3 {
            x: 3.0,
            y: 1.0,
            z: 1.0,
        });
        assert_eq!(true, ball_aabb0.intersects(&ball_aabb1));
        assert_eq!(true, ball_aabb0.intersects(&ball_aabb2));
        assert_eq!(false, ball_aabb0.intersects(&ball_aabb3));
    }

    #[test]
    fn test_contains() {
        let ball = Ball::new(1.0).unwrap();
        let bigball = Ball::new(3.0).unwrap();
        let ball_aabb0 = ball.local_bounding_volume();
        let ball_aabb1 = ball.bounding_volume(Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        });
        let ball_aabb2 = ball.bounding_volume(Vector3 {
            x: 1.5,
            y: 1.5,
            z: 1.5,
        });
        let ball_aabb3 = ball.bounding_volume(Vector3 {
            x: 3.0,
            y: 1.0,
            z: 1.0,
        });
        let bigball_aabb = bigball.local_bounding_volume();
        assert_eq!(false, ball_aabb0.contains(&ball_aabb1));
        assert_eq!(true, bigball_aabb.contains(&ball_aabb2));
        assert_eq!(false, bigball_aabb.contains(&ball_aabb3));
    }

    #[test]
    fn test_merged() {
        let ball = Ball::new(1.0).unwrap();
        let bigball = Ball::new(3.0).unwrap();
        let ball_aabb = ball.bounding_volume(Vector3 {
            x: 3.0,
            y: 1.0,
            z: 1.0,
        });
        let bigball_aabb = bigball.local_bounding_volume();
        assert_eq!(
            ball_aabb.merged(&bigball_aabb),
            AABB {
                mins: Vector3 {
                    x: -3.0,
                    y: -3.0,
                    z: -3.0
                },
                maxs: Vector3 {
                    x: 4.0,
                    y: 3.0,
                    z: 3.0
                },
            }
        );
    }
}
