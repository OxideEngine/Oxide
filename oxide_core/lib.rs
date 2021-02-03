use std::ops;

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
        /* TODO: Can be implemented only after Matrix multiplication implemented */
        Matrix {}
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

#[cfg(test)]
mod vectors {
    use super::*;

    #[test]
    fn test_length() {
        let vector: Vector = Vector { x: 3.0, y: 4.0 };
        /* TODO: Change order of args */
        assert_eq!(5.0, vector.length());
    }

    #[test]
    fn test_add() {
        let vector1: Vector = Vector { x: 3.0, y: 4.0 };
        let vector2: Vector = Vector { x: 1.0, y: 2.0 };

        let vector3 = vector1 + vector2;

        assert_eq!(vector3.x, 4.0);
        assert_eq!(vector3.y, 6.0);
    }
}
