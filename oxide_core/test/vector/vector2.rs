#[cfg(test)]
mod test {
    use crate::vector::*;

    fn assert_vector2_approx_eq(a: &vector2::Vector2, b: &vector2::Vector2) {
        assert_approx_eq!(a.x, b.x);
        assert_approx_eq!(a.y, b.y);
    }

    #[test]
    fn get_squared_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar: f32 = 25.0;
        assert_approx_eq!(tar, new_vector.get_squared_length());
    }

    #[test]
    fn get_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar: f32 = 5.0;
        assert_approx_eq!(tar, new_vector.get_length());
    }

    #[test]
    fn inner_product() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar: f32 = 37.0;
        assert_approx_eq!(tar, new_vector_1.inner_product(&new_vector_2));
    }

    #[test]
    fn scale_3() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar_vector = vector2::Vector2 { x: 9.0, y: 12.0 };
        assert_eq!(tar_vector, new_vector.scale(3.0));
    }

    #[test]
    fn add() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar_vector = vector2::Vector2 { x: 6.0, y: 11.0 };
        assert_eq!(tar_vector, new_vector_1 + new_vector_2);
    }

    #[test]
    fn sub() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar_vector = vector2::Vector2 { x: 0.0, y: -3.0 };
        assert_eq!(tar_vector, new_vector_1 - new_vector_2);
    }

    #[test]
    fn normalize() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar_vector = vector2::Vector2 { x: 0.6, y: 0.8 };
        let res_vector = new_vector.normalize();
        assert_vector2_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(1.0, res_vector.get_length());
    }
}
