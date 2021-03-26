use crate::traits::Float;
use crate::vector::*;
use crate::vector3::Vector3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Quaternion<T>
where
    T: Float,
{
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

trait VectorPart<T: Float> {
    fn get_vector_part(&self) -> Vector3<T>;
}

impl<T> VectorPart<T> for Quaternion<T>
where
    T: Float,
{
    fn get_vector_part(&self) -> Vector3<T> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

trait ScalarPart<T: Float> {
    fn get_scalar_part(&self) -> T;
}

impl<T> ScalarPart<T> for Quaternion<T>
where
    T: Float,
{
    fn get_scalar_part(&self) -> T {
        self.w
    }
}

pub fn id<T>() -> Quaternion<T>
where
    T: Float,
{
    let one = T::one();
    let zero = T::zero();
    Quaternion {
        w: one,
        x: zero,
        y: zero,
        z: zero,
    }
}

pub fn add<T>(a: Quaternion<T>, b: Quaternion<T>) -> Quaternion<T>
where
    T: Float,
{
    Quaternion {
        w: a.w + b.w,
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

pub fn scale<T>(q: Quaternion<T>, t: T) -> Quaternion<T>
where
    T: Float,
{
    Quaternion {
        w: q.w * t,
        x: q.x * t,
        y: q.y * t,
        z: q.z * t,
    }
}

pub fn mul<T>(a: Quaternion<T>, b: Quaternion<T>) -> Quaternion<T>
where
    T: Float,
{
    let vector_a = a.get_vector_part();
    let vector_b = b.get_vector_part();

    let inner_product_result = vector_a.inner_product(vector_b);
    let vector_part = vector_b
        .scale(a.w)
        .add(vector_a.scale(b.w))
        .add(vector_a.outer_product(vector_b));

    Quaternion {
        w: a.w * b.w - inner_product_result,
        x: vector_part.x,
        y: vector_part.y,
        z: vector_part.z,
    }
}

pub fn conj<T>(a: Quaternion<T>) -> Quaternion<T>
where
    T: Float,
{
    let vector_neg = a.get_vector_part().neg();

    Quaternion {
        w: a.w,
        x: vector_neg.x,
        y: vector_neg.y,
        z: vector_neg.z,
    }
}

pub fn square_len<T>(q: Quaternion<T>) -> T
where
    T: Float,
{
    q.w * q.w + q.get_vector_part().get_squared_length()
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
    let vector_q = q.get_vector_part();
    let two = T::one() + T::one();
    let t: Vector3<T> = vector_q.outer_product(v).scale(two);
    v.add(t.scale(q.w)).add(vector_q.outer_product(t))
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

    let vector_part = Vector3 {
        x: sin_x_2 * cos_y_2 * cos_z_2 + cos_x_2 * sin_y_2 * sin_z_2,
        y: cos_x_2 * sin_y_2 * cos_z_2 + sin_x_2 * cos_y_2 * sin_z_2,
        z: cos_x_2 * cos_y_2 * sin_z_2 + sin_x_2 * sin_y_2 * cos_z_2,
    };

    Quaternion {
        w: cos_x_2 * cos_y_2 * cos_z_2 + sin_x_2 * sin_y_2 * sin_z_2,
        x: vector_part.x,
        y: vector_part.y,
        z: vector_part.z,
    }
}

pub fn axis_angle<T>(axis: Vector3<T>, angle: T) -> Quaternion<T>
where
    T: Float,
{
    let two: T = T::one() + T::one();
    let half_angle = angle / two;
    let vector_part = axis.scale(half_angle.sin());
    Quaternion {
        w: half_angle.cos(),
        x: vector_part.x,
        y: vector_part.y,
        z: vector_part.z,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id() {
        let id = id();
        let idq: Quaternion<f32> = Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(id, idq);
    }

    #[test]
    fn test_get_vector_part() {
        let id = id();
        let zero_vector = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(id.get_vector_part(), zero_vector);
    }

    #[test]
    fn test_add() {
        let a: Quaternion<f32> = Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let b: Quaternion<f32> = Quaternion {
            w: 0.0,
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(
            add(a, b),
            Quaternion {
                w: 1.0,
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn test_scale() {
        let q: Quaternion<f32> = Quaternion {
            w: 1.0,
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let t = 2.0f32;
        let result: Quaternion<f32> = Quaternion {
            w: 2.0,
            x: 4.0,
            y: 6.0,
            z: 8.0,
        };
        assert_eq!(scale(q, t), result);
    }

    #[test]
    fn test_axis_angle() {
        let axis: Vector3<f32> = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let q: Quaternion<f32> = axis_angle(axis.normalized(), ::std::f32::consts::PI);

        assert!((square_len(q) - 1.0).abs() <= ::std::f32::EPSILON);
    }
}
