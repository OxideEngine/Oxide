use crate::traits::Float;
use vecmath::Vector3;

pub type Quaternion<T> = (T, [T; 3]);

pub fn id<T>() -> Quaternion<T>
where
    T: Float,
{
    let one = T::one();
    let zero = T::zero();
    (one, [zero, zero, zero])
}

pub fn add<T>(a: Quaternion<T>, b: Quaternion<T>) -> Quaternion<T>
where
    T: Float,
{
    use vecmath::vec3_add as add;
    (a.0 + b.0, add(a.1, b.1))
}

pub fn scale<T>(q: Quaternion<T>, t: T) -> Quaternion<T>
where
    T: Float,
{
    use vecmath::vec3_scale as scale;
    (q.0 * t, scale(q.1, t))
}

pub fn mul<T>(a: Quaternion<T>, b: Quaternion<T>) -> Quaternion<T>
where
    T: Float,
{
    use vecmath::vec3_add as add;
    use vecmath::vec3_cross as cross;
    use vecmath::vec3_dot as dot;
    use vecmath::vec3_scale as scale;

    (
        a.0 * b.0 - dot(a.1, b.1),
        add(add(scale(b.1, a.0), scale(a.1, b.0)), cross(a.1, b.1)),
    )
}

pub fn conj<T>(a: Quaternion<T>) -> Quaternion<T>
where
    T: Float,
{
    use vecmath::vec3_neg as neg;

    (a.0, neg(a.1))
}

pub fn square_len<T>(q: Quaternion<T>) -> T
where
    T: Float,
{
    use vecmath::vec3_square_len as square_len;
    q.0 * q.0 + square_len(q.1)
}

pub fn len<T>(q: Quaternion<T>) -> T
where
    T: Float,
{
    square_len(q).sqrt()
}

pub fn rotate_vector<T>(q: Quaternion<T>, v: Vector3<T>) -> Vector3<T>
where
    T: Float,
{
    use vecmath::{vec3_add as add, vec3_cross as cross, vec3_scale as scale};
    let two = T::one() + T::one();
    let t: Vector3<T> = scale(cross(q.1, v), two);
    add(add(v, scale(t, q.0)), cross(q.1, t))
}

pub fn euler_angles<T>(x: T, y: T, z: T) -> Quaternion<T>
where
    T: Float,
{
    let two: T = T::one() + T::one();

    let half_x = x / two;
    let half_y = y / two;
    let half_z = z / two;

    let cos_x_2 = half_x.cos();
    let cos_y_2 = half_y.cos();
    let cos_z_2 = half_z.cos();

    let sin_x_2 = half_x.sin();
    let sin_y_2 = half_y.sin();
    let sin_z_2 = half_z.sin();

    (
        cos_x_2 * cos_y_2 * cos_z_2 + sin_x_2 * sin_y_2 * sin_z_2,
        [
            sin_x_2 * cos_y_2 * cos_z_2 + cos_x_2 * sin_y_2 * sin_z_2,
            cos_x_2 * sin_y_2 * cos_z_2 + sin_x_2 * cos_y_2 * sin_z_2,
            cos_x_2 * cos_y_2 * sin_z_2 + sin_x_2 * sin_y_2 * cos_z_2,
        ],
    )
}

pub fn axis_angle<T>(axis: Vector3<T>, angle: T) -> Quaternion<T>
where
    T: Float,
{
    use vecmath::vec3_scale as scale;
    let two: T = T::one() + T::one();
    let half_angle = angle / two;
    (half_angle.cos(), scale(axis, half_angle.sin()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id() {
        let id = id();
        let idq: Quaternion<f32> = (1.0, [0.0, 0.0, 0.0]);
        assert_eq!(id, idq);
    }

    #[test]
    fn test_add() {
        let a: Quaternion<f32> = (1.0, [0.0, 0.0, 0.0]);
        let b: Quaternion<f32> = (0.0, [1.0, 1.0, 1.0]);
        assert_eq!(add(a, b), (1.0, [1.0, 1.0, 1.0]));
    }

    #[test]
    fn test_scale() {
        let q: Quaternion<f32> = (1.0, [2.0, 3.0, 4.0]);
        let t = 2.0f32;
        let result: Quaternion<f32> = (2.0, [4.0, 6.0, 8.0]);
        assert_eq!(scale(q, t), result);
    }
}
