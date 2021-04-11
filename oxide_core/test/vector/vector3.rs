#[cfg(test)]
mod test {
    use crate::test::utils::*;
    use crate::vector::*;

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
            x: 0.42426407871192851464050661726291,
            y: 0.56568542494923801952067548968388,
            z: 0.70710678118654752440084436210485,
        };
        let res_vector = new_vector.normalize();
        assert_vector3_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(1.0, res_vector.get_length());
    }
}
