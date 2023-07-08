use crate::aabb::AABB;
use crate::collide_broad_phase::HasBoundingVolume;
use crate::shape::Ball;
use oxide_math::commons::vector3::Vector3;

pub fn ball_aabb(center: Vector3, radius: f32) -> AABB {
    AABB::new(
        Vector3 {
            x: center.x - radius,
            y: center.y - radius,
            z: center.z - radius,
        },
        Vector3 {
            x: center.x + radius,
            y: center.y + radius,
            z: center.z + radius,
        },
    )
}

pub fn local_ball_aabb(radius: f32) -> AABB {
    ball_aabb(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius,
    )
}

impl HasBoundingVolume<AABB> for Ball {
    fn bounding_volume(&self, tv: Vector3) -> AABB {
        ball_aabb(tv, self.radius())
    }

    fn local_bounding_volume(&self) -> AABB {
        local_ball_aabb(self.radius())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_aabb() {
        let ball = Ball::new(4.0).unwrap();
        let ball_aabb = ball.bounding_volume(Vector3 { x: 3.0, y: 4.0, z: 5.0 });
        assert_eq!(ball_aabb, AABB {
            mins: Vector3 { x: -1.0, y: 0.0, z: 1.0 },
            maxs: Vector3 { x: 7.0, y: 8.0, z: 9.0 },
        });
        
    }

    #[test]
    fn test_local_aabb() {
        let ball = Ball::new(4.0).unwrap();
        let ball_aabb = ball.local_bounding_volume();
        assert_eq!(ball_aabb, AABB {
            mins: Vector3 { x: -4.0, y: -4.0, z: -4.0 },
            maxs: Vector3 { x: 4.0, y: 4.0, z: 4.0 },
        });
    }
}
