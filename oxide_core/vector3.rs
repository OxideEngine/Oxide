use crate::vector;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl vector::Length for Vector3 {
    fn get_squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn get_length(&self) -> f32 {
        self.get_squared_length().sqrt()
    }
}

impl vector::InnerProduct for Vector3 {
    fn inner_product(&self, _rhs) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y
    }
}
