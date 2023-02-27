use oxide_math::commons::vector::*;
use oxide_math::commons::vector3::Vector3;
use num_traits::pow;

pub struct AABB {
    pub mins: Vector3,
    pub maxs: Vector3,
}

pub fn aabb(s: &S) -> AABB 
where
    S: HasBoundingVolume
{
    s.local_bounding_volume()        
}

impl AABB {
    pub fn new(mins: Vector3, maxs: Vector3) -> AABB {
        AABB { mins: mins, maxs: maxs }
    }

    pub fn mins(&self) -> &Vector3 {
        &self.mins
    }
 
    pub fn maxs(&self) -> &Vector3 {
        &self.maxs
    }   
}

impl BoundingVolume for AABB {
    fn intersects(&self, other: &AABB) -> bool {
        self.mins.x <= other.maxs.x &&
        self.maxs.x >= other.maxs.x &&
        self.mins.y <= other.maxs.y &&
        self.maxs.y >= other.maxs.y &&
        self.mins.z <= other.maxs.z &&
        self.maxs.z >= other.maxs.z
    }
}
