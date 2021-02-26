use crate::vector;

pub struct Vector2 {
    x: i32,
    y: i32,
}

impl vector::LengthComputable for Vector2 {
    fn get_length(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}
