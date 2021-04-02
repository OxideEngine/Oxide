pub mod matrix;
pub mod vector;

#[cfg(test)]
mod test {
    use super::vector::*;

    #[test]
    fn test_vector2_get_squared_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(25.0, new_vector.get_squared_length());
    }

    #[test]
    fn test_vector2_get_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(5.0, new_vector.get_length());
    }
}
