pub struct Vector {
    pub x: f64,
    pub y: f64,
}

trait VectorTrait {
    fn length(&self) -> f64;
}

impl VectorTrait for Vector {
    fn length(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }
}
