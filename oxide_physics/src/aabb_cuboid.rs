use crate::aabb::AABB;
use crate::collide_broad_phase::HasBoundingVolume;
use crate::shape::Cuboid;
use oxide_math::commons::vector3::Vector3;

impl HasBoundingVolume<AABB> for Cuboid {
    fn bounding_volume(&self, tv: Vector3) -> AABB {
        let tv2 = Vector3 {
            x: tv.x,
            y: tv.y,
            z: tv.z,
        };
        AABB::new(self.mins() + tv, self.maxs() + tv2)
    }

    fn local_bounding_volume(&self) -> AABB {
        self.bounding_volume(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuboid_aabb() {
        let cuboid = Cuboid::new(Vector3 { x: 3.0, y: 4.0, z: 5.0 }).unwrap();
        let cuboid_aabb = cuboid.bounding_volume(Vector3 { x: 3.0, y: 4.0, z: 5.0 });
        assert_eq!(cuboid_aabb, AABB {
            mins: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            maxs: Vector3 { x: 6.0, y: 8.0, z: 10.0 },
        });
    }

    #[test]
    fn test_local_aabb() {
        let cuboid = Cuboid::new(Vector3 { x: 3.0, y: 4.0, z: 5.0 }).unwrap();
        let cuboid_aabb = cuboid.local_bounding_volume();
        assert_eq!(cuboid_aabb, AABB {
            mins: Vector3 { x: -3.0, y: -4.0, z: -5.0 },
            maxs: Vector3 { x: 3.0, y: 4.0, z: 5.0 },
        });
    }
}
