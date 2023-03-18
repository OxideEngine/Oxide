use oxide_math::commons::vector3::Vector3;

pub trait BoundingVolume {
    // check if the bounding volume 'bv' intersects with self
    fn intersects(&self, bv: &Self) -> bool;

    // check if self contains the 'bv'
    fn contains(&self, bv: &Self) -> bool;

    // merge this bounding volume with the other 'bv'
    fn merged(&self, bv: &Self) -> Self;
}

pub trait HasBoundingVolume<BV> {
    // TBD: rotation by 4x4 matrix
    // bounding volume of 'self' translated by 'tv'
    fn bounding_volume(&self, tv: Vector3) -> BV;

    fn local_bounding_volume(&self) -> BV {
        self.bounding_volume(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}
