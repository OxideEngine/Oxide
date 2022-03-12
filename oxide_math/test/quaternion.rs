#[cfg(test)]
mod test {
    use crate::commons::quaternion::*;
    use crate::commons::vector::*;
    use crate::commons::vector3::Vector3;

    #[test]
    fn test_set_identity() {
        let id = make_identity();
        let mut q = Quaternion {
            x: 3.0,
            y: 2.0,
            z: 1.0,
            w: 0.0,
        };
        q.set_identity();
        assert_eq!(id, q);
    }

    #[test]
    fn test_make_identity() {
        let id = make_identity();
        let idq: Quaternion = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };
        assert_eq!(id, idq);
    }

    #[test]
    fn test_get_scalar_part() {
        let q = Quaternion {
            x: 2.0,
            y: 1.0,
            z: 0.0,
            w: 3.0,
        };
        assert_approx_eq!(q.get_scalar_part(), 3.0);
    }

    #[test]
    fn test_get_vector_part() {
        let id = make_identity();
        let zero_vector = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(id.get_vector_part(), zero_vector);
    }

    #[test]
    fn test_add() {
        let a: Quaternion = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };
        let b: Quaternion = Quaternion {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 0.0,
        };
        assert_eq!(
            a + b,
            Quaternion {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            }
        );
    }

    #[test]
    fn test_scale() {
        let q: Quaternion = Quaternion {
            x: 2.0,
            y: 3.0,
            z: 4.0,
            w: 1.0,
        };
        let t = 2.0f32;
        let result: Quaternion = Quaternion {
            x: 4.0,
            y: 6.0,
            z: 8.0,
            w: 2.0,
        };
        assert_eq!(q.scale(t), result);
    }

    #[test]
    fn test_mul() {
        let a = Quaternion {
            x: 2.0,
            y: -1.0,
            z: -3.0,
            w: 1.0,
        };
        let b = Quaternion {
            x: 3.0,
            y: -2.0,
            z: 4.0,
            w: -1.0,
        };
        let result = Quaternion {
            x: -9.0,
            y: -18.0,
            z: 6.0,
            w: 3.0,
        };
        assert_eq!(a * b, result);
    }

    #[test]
    fn test_dot() {
        let a = Quaternion {
            x: 2.0,
            y: -1.0,
            z: -3.0,
            w: 1.0,
        };
        let b = Quaternion {
            x: 3.0,
            y: -2.0,
            z: 4.0,
            w: -1.0,
        };
        assert_approx_eq!(dot(a, b), -1.0 + -4.0)
    }

    #[test]
    fn test_inverse() {
        let q = Quaternion {
            x: -3.0,
            y: 2.0,
            z: -6.0,
            w: 2.0,
        };
        let a = inverse(q);
        assert_approx_eq!(a.x, -3.0 / (53.0f32.sqrt()));
        assert_approx_eq!(a.y, 2.0 / (53.0f32.sqrt()));
        assert_approx_eq!(a.z, -6.0 / (53.0f32.sqrt()));
        assert_approx_eq!(a.w, 2.0 / (53.0f32.sqrt()));
    }

    #[test]
    fn test_conj() {
        let q = Quaternion {
            x: 2.0,
            y: 3.0,
            z: 4.0,
            w: 1.0,
        };
        let t = Quaternion {
            x: -2.0,
            y: -3.0,
            z: -4.0,
            w: 1.0,
        };
        assert_eq!(conjugate(q), t);
    }

    #[test]
    fn test_from_euler_angle() {
        let q = from_euler_angles(
            ::std::f32::consts::FRAC_PI_6,
            ::std::f32::consts::FRAC_PI_3,
            ::std::f32::consts::FRAC_PI_2,
        );
        assert_approx_eq!(q.x, -0.1830127);
        assert_approx_eq!(q.y, 0.5);
        assert_approx_eq!(q.z, 0.5);
        assert_approx_eq!(q.w, 0.6830127);
    }

    #[test]
    fn test_from_axis_angle() {
        let axis = Vector3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let q = from_axis_angle(axis.normalize(), ::std::f32::consts::FRAC_PI_3);
        assert_approx_eq!(q.x, 0.4008919);
        assert_approx_eq!(q.y, 0.2672612);
        assert_approx_eq!(q.z, 0.1336306);
        assert_approx_eq!(q.w, 0.8660254);
    }

    #[test]
    fn test_get_axis_angle() {
        let q = Quaternion {
            x: 0.4008919,
            y: 0.2672612,
            z: 0.1336306,
            w: 0.8660254,
        };
        let (axis, angle) = get_axis_angle(q);
        assert_approx_eq!(axis.x, 3.0 / (14.0f32).sqrt());
        assert_approx_eq!(axis.y, (2.0 / 7.0f32).sqrt());
        assert_approx_eq!(axis.z, 1.0 / (14.0f32).sqrt());
        assert_approx_eq!(angle, ::std::f32::consts::FRAC_PI_3);
    }

    #[test]
    fn test_get_axis_angle_over_pi() {
        let q = Quaternion {
            x: 0.4008919,
            y: 0.2672612,
            z: 0.1336306,
            w: -0.1,
        };
        let (axis, angle) = get_axis_angle(q);
        assert_approx_eq!(axis.x, -(3.0 / (14.0f32).sqrt()));
        assert_approx_eq!(axis.y, -((2.0 / 7.0f32).sqrt()));
        assert_approx_eq!(axis.z, -(1.0 / (14.0f32).sqrt()));
        assert_approx_eq!(angle, 2.941257);
    }

    #[test]
    fn test_rotate_vector() {
        let q = from_axis_angle(
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            ::std::f32::consts::FRAC_PI_2,
        );
        let rotated = rotate(q, ::std::f32::consts::FRAC_PI_2);
        let result = from_axis_angle(
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            ::std::f32::consts::PI,
        );
        assert_approx_eq!(rotated.x, result.x);
        assert_approx_eq!(rotated.y, result.y);
        assert_approx_eq!(rotated.z, result.z);
    }
}
