#[cfg(test)]
mod test {
    use crate::test::utils::*;
    use crate::vector::*;
    use crate::vector3;

    #[test]
    fn get_squared_length() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let tar: f32 = 50.0;
        let res = new_vector.get_squared_length();
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn get_length() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let tar: f32 = 7.071_068;
        let res = new_vector.get_length();
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn inner_product() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
        };
        let tar: f32 = 92.0;
        let res = new_vector_1.inner_product(&new_vector_2);
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn scale_3() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let tar_vector = vector3::Vector3 {
            x: 9.0,
            y: 12.0,
            z: 15.0,
        };
        let res_vector = new_vector.scale(3.0);
        assert_vector3_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn negate() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let tar_vector = vector3::Vector3 {
            x: -3.0,
            y: -4.0,
            z: -5.0,
        };
        let res_vector = new_vector.negate();
        assert_vector3_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(tar_vector.get_length(), res_vector.get_length());
    }

    #[test]
    fn distance1() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
        };
        let target = 6.708_204;
        let result = new_vector_1.distance(&new_vector_2);
        assert_approx_eq!(target, result);
    }

    #[test]
    fn distance2() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: -3.0,
            y: -4.0,
            z: -5.0,
        };
        let target = 14.142_136;
        let result = new_vector_1.distance(&new_vector_2);
        assert_approx_eq!(target, result);
    }

    #[test]
    fn clamp() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let tar_vector = vector3::Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        };
        let res_vector = new_vector.clamp(0.0, 3.0);
        assert_vector3_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn project_on_to() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
        };
        let tar_vector = vector3::Vector3 {
            x: 1.541_899_44,
            y: 3.597_765_36,
            z: 5.653_631_28,
        };
        let res_vector = new_vector_1.project_on_to(&new_vector_2);
        assert_vector3_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn add() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
        };
        let tar_vector = vector3::Vector3 {
            x: 6.0,
            y: 11.0,
            z: 16.0,
        };
        let res_vector = new_vector_1 + new_vector_2;
        assert_vector3_approx_eq(&tar_vector, &res_vector);
    }
    #[test]
    fn sub() {
        let new_vector_1 = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let new_vector_2 = vector3::Vector3 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
        };
        let tar_vector = vector3::Vector3 {
            x: 0.0,
            y: -3.0,
            z: -6.0,
        };
        let res_vector = new_vector_1 - new_vector_2;
        assert_vector3_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn normalize() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let tar_vector = vector3::Vector3 {
            x: 0.424_264_07,
            y: 0.565_685_45,
            z: 0.707_106_77,
        };
        let res_vector = new_vector.normalize();
        assert_vector3_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(1.0, res_vector.get_length());
    }
}
