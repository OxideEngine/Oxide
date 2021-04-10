#[cfg(test)]
mod test {
    use crate::vector::*;

    fn assert_vector4_approx_eq(a: &vector4::Vector4, b: &vector4::Vector4) {
        assert_approx_eq!(a.x, b.x);
        assert_approx_eq!(a.y, b.y);
        assert_approx_eq!(a.z, b.z);
        assert_approx_eq!(a.w, b.w);
    }

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
        let tar: f32 = 9.2736184954957037525164160739902;
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
            x: 0.32349831961031524718080521188338,
            y: 0.43133109281375366290774028251117,
            z: 0.53916386601719207863467535313896,
            w: 0.64699663922063049436161042376676,
        };
        let res_vector = new_vector.normalize();
        assert_vector4_approx_eq(&tar_vector, &res_vector);
        assert_approx_eq!(1.0, res_vector.get_length());
    }
}