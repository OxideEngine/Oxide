use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    length: f64,
}

pub struct Matrix {}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            length: (x * x + y * y).sqrt(),
        }
    }

    pub fn length(&self) -> f64 {
        *(&self.get_length())
    }
}

trait LengthComputable {
    fn get_length(&self) -> f64;
}

trait Rotatable {
    fn rotate(&self, rotation_matrix: &Matrix) -> Matrix;
    fn get_inner_angle(self, _rhs: Vector) -> f64;
}

impl LengthComputable for Vector {
    fn get_length(&self) -> f64 {
        let squared_length = self.x * self.x + self.y * self.y;
        squared_length.sqrt()
    }
}

impl Rotatable for Vector {
    fn rotate(&self, rotation_matrix: &Matrix) -> Matrix {
        /* TODO: Can be implemented only after Matrix multiplication is implemented */
        Matrix {}
    }

    fn get_inner_angle(self, _rhs: Vector) -> f64 {
        (self * _rhs) / (self.get_length() * _rhs.get_length())
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        Vector::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, _rhs: Vector) -> f64 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

#[cfg(test)]
mod vectors {
    use super::*;

    #[test]
    fn test_length() {
        let vector: Vector = Vector::new(3.0, 4.0);
        assert_eq!(vector.length, 5.0);
    }

    #[test]
    fn test_get_length() {
        let vector: Vector = Vector::new(3.0, 4.0);
        assert_eq!(vector.get_length(), 5.0);
    }

    #[test]
    fn test_add() {
        let vector1: Vector = Vector::new(3.0, 4.0);
        let vector2: Vector = Vector::new(1.0, 2.0);

        let add_result = vector1 + vector2;

        assert_eq!(add_result.x, 4.0);
        assert_eq!(add_result.y, 6.0);
    }

    #[test]
    fn test_inner_product() {
        let vector1: Vector = Vector::new(3.0, 4.0);
        let vector2: Vector = Vector::new(1.0, 2.0);

        let inner_product = vector1 * vector2;

        assert_eq!(11.0, inner_product);
    }

    #[test]
    fn test_get_inner_angle() {
        let vector1: Vector = Vector::new(3.0, 4.0);
        let vector2: Vector = Vector::new(1.0, 2.0);

        let inner_angle = Vector::get_inner_angle(vector1, vector2);

        assert_eq!(0.9838699100999074, inner_angle);
    }
}
