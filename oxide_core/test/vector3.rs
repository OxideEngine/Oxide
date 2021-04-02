#[cfg(test)]
mod test {
    use crate::vector::*;

    #[test]
    fn get_squared_length() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let res: f32 = 50.0;
        assert_approx_eq!(res, new_vector.get_squared_length());
    }

    #[test]
    fn get_length() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let res: f32 = 7.071_068;
        assert_approx_eq!(res, new_vector.get_length());
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
        let res: f32 = 92.0;
        assert_approx_eq!(res, new_vector_1.inner_product(&new_vector_2));
    }

    #[test]
    fn scale_3() {
        let new_vector = vector3::Vector3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let res_vector = vector3::Vector3 {
            x: 9.0,
            y: 12.0,
            z: 15.0,
        };
        assert_eq!(res_vector, new_vector.scale(3.0));
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
        let res_vector = vector3::Vector3 {
            x: 6.0,
            y: 11.0,
            z: 16.0,
        };
        assert_eq!(res_vector, new_vector_1 + new_vector_2);
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
        let res_vector = vector3::Vector3 {
            x: 0.0,
            y: -3.0,
            z: -6.0,
        };
        assert_eq!(res_vector, new_vector_1 - new_vector_2);
    }
}
