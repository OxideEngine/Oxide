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
