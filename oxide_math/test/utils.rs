use crate::commons::{vector2, vector3, vector4};

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let eps = 1.0e-6;
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
}

pub fn assert_vector2_approx_eq(a: &vector2::Vector2, b: &vector2::Vector2) {
    assert_approx_eq!(a.x, b.x);
    assert_approx_eq!(a.y, b.y);
}

pub fn assert_vector3_approx_eq(a: &vector3::Vector3, b: &vector3::Vector3) {
    assert_approx_eq!(a.x, b.x);
    assert_approx_eq!(a.y, b.y);
    assert_approx_eq!(a.z, b.z);
}

pub fn assert_vector4_approx_eq(a: &vector4::Vector4, b: &vector4::Vector4) {
    assert_approx_eq!(a.x, b.x);
    assert_approx_eq!(a.y, b.y);
    assert_approx_eq!(a.z, b.z);
    assert_approx_eq!(a.w, b.w);
}
