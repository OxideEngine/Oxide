pub mod matrix;
pub mod vector;
pub mod vector2;
pub mod vector3;

#[cfg(test)]
mod test {
    use super::vector::*;
    use super::*;

    #[test]
    fn test_vector2_get_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(5.0, new_vector.get_length())
    }
}
