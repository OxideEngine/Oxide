use crate::matrix::*;

pub type Matrix2x2 = [[f32; 2]; 2];

impl Fill for Matrix2x2 {
    fn fill(number: f32) -> Self {
        [[number; 2]; 2]
    }
}

impl SetDiagonal for Matrix2x2 {
    fn set_diagonal(&self, number: f32) -> Self {
        self[0][0] = number;
        self[1][1] = number;
        *self
    }
}

impl IsSimilar for Matrix2x2 {
    fn is_similar(&self, other: Matrix2x2) -> bool {
        panic!("TODO: implement")
    }
}

impl Get<[f32; 2]> for Matrix2x2 {
    fn row_length(&self) -> usize {
        2
    }
    fn col_length(&self) -> usize {
        2
    }
    fn get_row(&self, index: usize) -> [f32; 2] {
        self[index]
    }
    fn get_col(&self, index: usize) -> [f32; 2] {
        [self[0][index], self[1][index]]
    }
}

impl Transpose for Matrix2x2 {
    fn transpose(&self) -> Self {
        [[self[0][0], self[1][0]], [self[0][1], self[1][1]]]
    }
}

impl Min for Matrix2x2 {
    fn min(&self) -> f32 {
        self.into_iter()
            .flatten()
            .fold(std::f32::MAX, |a, b| a.min(*b))
    }
    fn abs_min(&self) -> f32 {
        self.into_iter()
            .flatten()
            .fold(std::f32::MAX, |a, b| a.abs().min(b.abs()))
    }
}

impl Max for Matrix2x2 {
    fn max(&self) -> f32 {
        self.into_iter()
            .flatten()
            .fold(std::f32::MIN, |a, b| a.max(*b))
    }
    fn abs_max(&self) -> f32 {
        self.into_iter()
            .flatten()
            .fold(std::f32::MIN, |a, b| a.abs().max(b.abs()))
    }
}

impl Trace for Matrix2x2 {
    fn trace(&self) -> f32 {
        self[0][0] + self[1][1]
    }
}

impl Determinant for Matrix2x2 {
    fn determinant(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Diagonal for Matrix2x2 {
    fn diagonal(&self) -> Self {
        panic!("TODO: implement")
    }
}

impl Inverse for Matrix2x2 {
    fn inverse(&self) -> Self {
        let det = self.determinant();
        [
            [self[1][1] / det, -self[0][1] / det],
            [-self[1][0] / det, self[0][0] / det],
        ]
    }
}

impl Sum for Matrix2x2 {
    fn sum(&self) -> f32 {
        self.into_iter().flatten().fold(0.0, |a, b| a + b)
    }
}

impl Average for Matrix2x2 {
    fn average(&self) -> f32 {
        self.sum() / 4.0
    }
}

impl MatrixHelper for Matrix2x2 {
    fn make_zero() -> Matrix2x2 {
        [[0., 0.], [0., 0.]]
    }
    fn make_identity() -> Matrix2x2 {
        Self::make_zero().set_diagonal(1.)
    }
    fn make_scale() -> Matrix2x2 {
        panic!("TODO: implement")
    }
    fn make_rotation(theta: f32) -> Matrix2x2 {
        [[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]
    }
    fn make_translation() -> Matrix2x2 {
        panic!("TODO: implement")
    }
}

impl Add for Matrix2x2 {
    fn add(self, _rhs: Matrix2x2) -> Matrix2x2 {
        [
            [self[0][0] + _rhs[0][0], self[0][1] + _rhs[0][1]],
            [self[1][0] + _rhs[1][0], self[1][1] + _rhs[1][1]],
        ]
    }
}

impl Sub for Matrix2x2 {
    fn sub(self, _rhs: Matrix2x2) -> Matrix2x2 {
        [
            [self[0][0] - _rhs[0][0], self[0][1] - _rhs[0][1]],
            [self[1][0] - _rhs[1][0], self[1][1] - _rhs[1][1]],
        ]
    }
}

impl Mul for Matrix2x2 {
    fn mul(self, _rhs: Matrix2x2) -> Matrix2x2 {
        [
            [
                self[0][0] * _rhs[0][0] + self[0][1] * _rhs[1][0],
                self[0][0] * _rhs[0][1] + self[0][1] * _rhs[1][1],
            ],
            [
                self[1][0] * _rhs[0][0] + self[1][1] * _rhs[1][0],
                self[1][0] * _rhs[0][1] + self[1][1] * _rhs[1][1],
            ],
        ]
    }
}

impl Div for Matrix2x2 {
    fn div(self, _rhs: Matrix2x2) -> Matrix2x2 {
        self.mul(_rhs.inverse())
    }
}
