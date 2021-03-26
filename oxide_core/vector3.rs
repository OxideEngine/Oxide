use crate::traits::Float;
use crate::vector;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> vector::Add for Vector3<T>
where
    T: Float,
{
    fn add(&self, _rhs: Self) -> Self {
        let x = self.x + _rhs.x;
        let y = self.y + _rhs.y;
        let z = self.z + _rhs.z;
        Vector3 { x: x, y: y, z: z }
    }
}

impl<T> vector::Length<T> for Vector3<T>
where
    T: Float,
{
    fn get_squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn get_length(&self) -> T {
        self.get_squared_length().sqrt()
    }
}

impl<T> vector::Normalized for Vector3<T>
where
    T: Float,
{
    fn normalized(&self) -> Self {
        let length = self.x * self.x + self.y * self.y + self.z * self.z;
        let squared_length = length.sqrt();
        Vector3 {
            x: self.x / squared_length,
            y: self.y / squared_length,
            z: self.z / squared_length,
        }
    }
}

impl<T> vector::InnerProduct<T> for Vector3<T>
where
    T: Float,
{
    fn inner_product(&self, _rhs: Self) -> T {
        self.x * _rhs.x + self.y * _rhs.y
    }
}

impl<T> vector::OuterProduct<T> for Vector3<T>
where
    T: Float,
{
    fn outer_product(&self, _rhs: Self) -> Self {
        Vector3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x,
        }
    }
}

impl<T> vector::Scale<T> for Vector3<T>
where
    T: Float,
{
    fn scale(&self, multiplier: T) -> Vector3<T> {
        Vector3 {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
        }
    }
}

impl<T> vector::Negate for Vector3<T>
where
    T: Float,
{
    fn neg(&self) -> Vector3<T> {
        Vector3 {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vector::*;

    #[test]
    fn test_add() {
        let vec = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let vec_rhs = Vector3 {
            x: 1.2,
            y: 1.3,
            z: 1.4,
        };
        let vec_res = Vector3 {
            x: 2.2,
            y: 2.3,
            z: 2.4,
        };
        assert_eq!(vec.add(vec_rhs), vec_res);
    }

    #[test]
    fn test_length() {
        let unit_vec = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        assert_eq!(unit_vec.get_length(), 1.0);

        let vec = Vector3 {
            x: 1.1,
            y: 2.4,
            z: 7.7,
        };
        assert_eq!(vec.get_squared_length(), 66.26);
    }
}
