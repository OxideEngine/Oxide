pub struct Vector {
    pub x: f64,
    pub y: f64,
}

trait VectorTrait {
    fn length(&self) -> f64;
}

impl VectorTrait for Vector {
    fn length(&self) -> f64 {
        let squared_length = self.x * self.x + self.y * self.y;
        squared_length.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let vector: Vector = Vector { x: 3.0, y: 5.0 };
        assert_eq!(5.0, vector.length());
    }
}
