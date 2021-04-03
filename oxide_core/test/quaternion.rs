#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_id() {
    let id = id();
    let idq: Quaternion = Quaternion {
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
    };
    assert_eq!(id, idq);
  }

  #[test]
  fn test_get_vector_part() {
    let id = id();
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
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
    };
    let b: Quaternion = Quaternion {
      w: 0.0,
      x: 1.0,
      y: 1.0,
      z: 1.0,
    };
    assert_eq!(
      add(a, b),
      Quaternion {
        w: 1.0,
        x: 1.0,
        y: 1.0,
        z: 1.0
      }
    );
  }

  #[test]
  fn test_scale() {
    let q: Quaternion = Quaternion {
      w: 1.0,
      x: 2.0,
      y: 3.0,
      z: 4.0,
    };
    let t = 2.0f32;
    let result: Quaternion = Quaternion {
      w: 2.0,
      x: 4.0,
      y: 6.0,
      z: 8.0,
    };
    assert_eq!(scale(q, t), result);
  }

  #[test]
  fn test_axis_angle() {
    let axis: Vector3 = Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    };
    let q: Quaternion = axis_angle(axis.normalized(), ::std::f32::consts::PI);

    assert!((square_len(q) - 1.0).abs() <= ::std::f32::EPSILON);
  }
}
