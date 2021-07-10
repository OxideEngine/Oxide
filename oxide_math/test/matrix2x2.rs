#[cfg(test)]
mod test {
    use crate::matrix::*;
    use crate::matrix2x2::Matrix2x2;

    #[test]
    fn fill() {
        let m = Matrix2x2::fill(1.);
        assert_eq!(m, [[1., 1.], [1., 1.]]);
    }

    #[test]
    fn set_diagonal() {
        let m = Matrix2x2::make_zero().set_diagonal(1.);
        assert_eq!(m, [[1., 0.], [0., 1.]]);
    }

    #[test]
    fn row_length() {
        assert_eq!(Matrix2x2::make_zero().row_length(), 2usize);
    }

    #[test]
    fn col_length() {
        assert_eq!(Matrix2x2::make_zero().col_length(), 2usize);
    }

    #[test]
    fn get_row() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.get_row(0), [1., 2.]);
        assert_eq!(m.get_row(1), [3., 4.]);
    }

    #[test]
    fn get_col() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.get_col(0), [1., 3.]);
        assert_eq!(m.get_col(1), [2., 4.]);
    }

    #[test]
    fn transpose() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.transpose(), [[1., 3.], [2., 4.]]);
    }

    #[test]
    fn min() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.min(), 1.);
    }

    #[test]
    fn abs_min() {
        let m = [[-1., -2.], [-3., -4.]];
        assert_eq!(m.abs_min(), -1.);
    }

    #[test]
    fn max() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.max(), 4.);
    }

    #[test]
    fn abs_max() {
        let m = [[-1., -2.], [-3., -4.]];
        assert_eq!(m.abs_max(), -4.);
    }

    #[test]
    fn trace() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.trace(), 5.);
    }

    #[test]
    fn determinant() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.determinant(), -2.);
    }

    #[test]
    fn inverse() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.inverse(), [[-2., 1.], [1.5, -0.5]]);
    }

    #[test]
    fn sum() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.sum(), 10.);
    }

    #[test]
    fn average() {
        let m = [[1., 2.], [3., 4.]];
        assert_eq!(m.average(), 2.5);
    }

    #[test]
    fn make_zero() {
        let m = Matrix2x2::make_zero();
        assert_eq!(m, [[0., 0.], [0., 0.]]);
    }

    #[test]
    fn make_identity() {
        let m = Matrix2x2::make_identity();
        assert_eq!(m, [[1., 0.], [0., 1.]]);
    }
}
