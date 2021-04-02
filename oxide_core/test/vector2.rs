#[cfg(test)]
mod test {
    use crate::vector::*;

    #[test]
    fn get_squared_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(25.0, new_vector.get_squared_length());
    }

    #[test]
    fn get_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(5.0, new_vector.get_length());
    }

    #[test]
    fn inner_product() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        assert_eq!(37.0, new_vector_1.inner_product(new_vector_2));
    }

    #[test]
    fn scale_3() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let res_vector = vector2::Vector2 { x: 9.0, y: 12.0 };
        assert_eq!(res_vector, new_vector.scale(3.0));
    }

    #[test]
    fn add() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let res_vector = vector2::Vector2 { x: 6.0, y: 11.0 };
        assert_eq!(res_vector, new_vector_1 + new_vector_2);
    }

    #[test]
    fn sub() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let res_vector = vector2::Vector2 { x: 0.0, y: -3.0 };
        assert_eq!(res_vector, new_vector_1 - new_vector_2);
    }
}
