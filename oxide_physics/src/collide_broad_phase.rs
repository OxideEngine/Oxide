
struct PotentialContact {
	body: RigidBody,
}

pub trait BoundingVolume {
    // check if the bounding volume intersects with another one
    fn intersects(&self, _: &Self) -> bool;
}

pub trait HasBoundingVolume {
    fn local_bounding_volume(&self) -> BV;
}
