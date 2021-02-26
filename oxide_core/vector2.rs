use crate::vector;

pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl vector::LengthComputable for Vector2 {
    fn get_length(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}
