
pub trait Shape {
    fn local_aabb(&self) -> AABB;
}

pub struct Ball {
    radius: f64,
}

impl Ball {
    pub fn new(radius: f64) -> Ball {
        assert!(radius > 0, "A ball radius must be positive.");

        Ball { radius }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
