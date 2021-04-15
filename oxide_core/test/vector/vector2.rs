#[cfg(test)]
mod test {
    use crate::test::utils::*;
    use crate::vector::*;

    #[test]
    fn get_squared_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar: f32 = 25.0;
        let res = new_vector.get_squared_length();
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn get_length() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar: f32 = 5.0;
        let res = new_vector.get_length();
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn inner_product() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar: f32 = 37.0;
        let res = new_vector_1.inner_product(&new_vector_2);
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn outer_product() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar_vector = vector3::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 9.0,
        };
        let res_vector = new_vector_1.outer_product(&new_vector_2);
        assert_vector3_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn scale_3() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar_vector = vector2::Vector2 { x: 9.0, y: 12.0 };
        let res_vector = new_vector.scale(3.0);
        assert_vector2_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn negate() {
        let new_vector = vector2::Vector2 { x: 3.0, y: 4.0 };
        let tar_vector = vector2::Vector2 { x: -3.0, y: -4.0 };
        let res_vector = new_vector.negate();
        assert_vector2_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(tar_vector.get_length(), res_vector.get_length());
    }

    #[test]
    fn add() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar_vector = vector2::Vector2 { x: 6.0, y: 11.0 };
        let res_vector = new_vector_1 + new_vector_2;
        assert_vector2_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn sub() {
        let new_vector_1 = vector2::Vector2 { x: 3.0, y: 4.0 };
        let new_vector_2 = vector2::Vector2 { x: 3.0, y: 7.0 };
        let tar_vector = vector2::Vector2 { x: 0.0, y: -3.0 };
        let res_vector = new_vector_1 - new_vector_2;
        assert_vector2_approx_eq(&tar_vector, &res_vector);
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
