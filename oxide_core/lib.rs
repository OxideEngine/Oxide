pub struct Vector {
    pub x: f64,
    pub y: f64,
}

pub struct Matrix {}

trait LengthComputable {
    fn length(&self) -> f64;
}

trait Rotatable {
    fn rotate(&self, rotation_matrix: &Matrix) -> Matrix;
}

impl LengthComputable for Vector {
    fn length(&self) -> f64 {
        let squared_length = self.x * self.x + self.y * self.y;
        squared_length.sqrt()
    }
}

impl Rotatable for Vector {
    fn rotate(&self, rotation_matrix: &Matrix) -> Matrix {
        Matrix {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let vector: Vector = Vector { x: 3.0, y: 4.0 };
        assert_eq!(5.0, vector.length());
    }
}
