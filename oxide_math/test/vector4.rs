#[cfg(test)]
mod test {
    use crate::test::utils::*;
    use crate::vector::*;
    use crate::vector4;

    #[test]
    fn get_squared_length() {
        let new_vector = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let tar: f32 = 86.0;
        let res = new_vector.get_squared_length();
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn get_length() {
        let new_vector = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let tar: f32 = 9.273_619;
        let res = new_vector.get_length();
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn inner_product() {
        let new_vector_1 = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let new_vector_2 = vector4::Vector4 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
            w: 13.0,
        };
        let tar: f32 = 170.0;
        let res = new_vector_1.inner_product(&new_vector_2);
        assert_approx_eq!(tar, res);
    }

    #[test]
    fn scale_3() {
        let new_vector = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let tar_vector = vector4::Vector4 {
            x: 9.0,
            y: 12.0,
            z: 15.0,
            w: 18.0,
        };
        let res_vector = new_vector.scale(3.0);
        assert_vector4_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn negate() {
        let new_vector = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let tar_vector = vector4::Vector4 {
            x: -3.0,
            y: -4.0,
            z: -5.0,
            w: -6.0,
        };
        let res_vector = new_vector.negate();
        assert_vector4_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(tar_vector.get_length(), res_vector.get_length());
    }

    #[test]
    fn distance1() {
        let new_vector_1 = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let new_vector_2 = vector4::Vector4 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
            w: 13.0,
        };
        let target = 9.695_359_71;
        let result = new_vector_1.distance(&new_vector_2);
        assert_approx_eq!(target, result);
    }

    #[test]
    fn distance2() {
        let new_vector_1 = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let new_vector_2 = vector4::Vector4 {
            x: -3.0,
            y: -4.0,
            z: -5.0,
            w: -6.0,
        };
        let target = 18.547_236_99;
        let result = new_vector_1.distance(&new_vector_2);
        assert_approx_eq!(target, result);
    }

    #[test]
    fn add() {
        let new_vector_1 = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let new_vector_2 = vector4::Vector4 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
            w: 13.0,
        };
        let tar_vector = vector4::Vector4 {
            x: 6.0,
            y: 11.0,
            z: 16.0,
            w: 19.0,
        };
        let res_vector = new_vector_1 + new_vector_2;
        assert_vector4_approx_eq(&tar_vector, &res_vector);
    }
    #[test]
    fn sub() {
        let new_vector_1 = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let new_vector_2 = vector4::Vector4 {
            x: 3.0,
            y: 7.0,
            z: 11.0,
            w: 13.0,
        };
        let tar_vector = vector4::Vector4 {
            x: 0.0,
            y: -3.0,
            z: -6.0,
            w: -7.0,
        };
        let res_vector = new_vector_1 - new_vector_2;
        assert_vector4_approx_eq(&tar_vector, &res_vector);
    }

    #[test]
    fn normalize() {
        let new_vector = vector4::Vector4 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
            w: 6.0,
        };
        let tar_vector = vector4::Vector4 {
            x: 0.323_498_3,
            y: 0.431_331_1,

            z: 0.539_163_9,
            w: 0.646_996_6,
        };
        let res_vector = new_vector.normalize();
        assert_vector4_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(1.0, res_vector.get_length());
    }
}
