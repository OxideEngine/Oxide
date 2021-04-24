use crate::vector::vector3::*;
use crate::vector::*;
use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Quaternion {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

pub trait VectorPart {
  fn get_vector_part(&self) -> Vector3;
}

impl VectorPart for Quaternion {
  fn get_vector_part(&self) -> Vector3 {
    Vector3 {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }
}

pub trait ScalarPart {
  fn get_scalar_part(&self) -> f32;
}

impl ScalarPart for Quaternion {
  fn get_scalar_part(&self) -> f32 {
    self.w
  }
}

pub fn id() -> Quaternion {
  let one = 1.0f32;
  let zero = 0.0f32;

  Quaternion {
    x: zero,
    y: zero,
    z: zero,
    w: one,
  }
}

impl Scale for Quaternion {
  fn scale(&self, t: f32) -> Self {
    Quaternion {
      x: self.x * t,
      y: self.y * t,
      z: self.z * t,
      w: self.w * t,
    }
  }
}

pub fn dot(a: Quaternion, b: Quaternion) -> f32 {
  a.get_scalar_part() * b.get_scalar_part()
    + a.get_vector_part().inner_product(&b.get_vector_part())
}

pub fn inverse(q: Quaternion) -> Quaternion {
  q.scale(q.get_length().recip())
}

pub fn conjugate(a: Quaternion) -> Quaternion {
  let vector_neg = a.get_vector_part().negate();
  Quaternion {
    x: vector_neg.x,
    y: vector_neg.y,
    z: vector_neg.z,
    w: a.w,
  }
}

impl Quaternion {
  pub fn get_squared_length(&self) -> f32 {
    self.w * self.w + self.get_vector_part().get_squared_length()
  }
  pub fn get_length(&self) -> f32 {
    self.get_squared_length().sqrt()
  }
}

pub fn rotate_vector(q: Quaternion, v: Vector3) -> Vector3 {
  let vector_q = q.get_vector_part();
  let t = vector_q.outer_product(&v).scale(2.0f32);
  v + t.scale(q.w) + vector_q.outer_product(&t)
}

pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Quaternion {
  let half_x = x / 2.0f32;
  let half_y = y / 2.0f32;
  let half_z = z / 2.0f32;

  let cos_x_2 = half_x.cos();
  let cos_y_2 = half_y.cos();
  let cos_z_2 = half_z.cos();

  let sin_x_2 = half_x.sin();
  let sin_y_2 = half_y.sin();
  let sin_z_2 = half_z.sin();

  // Z -> Y -> X
  Quaternion {
    x: sin_x_2 * cos_y_2 * cos_z_2 - cos_x_2 * sin_y_2 * sin_z_2,
    y: cos_x_2 * sin_y_2 * cos_z_2 + sin_x_2 * cos_y_2 * sin_z_2,
    z: cos_x_2 * cos_y_2 * sin_z_2 - sin_x_2 * sin_y_2 * cos_z_2,
    w: cos_x_2 * cos_y_2 * cos_z_2 + sin_x_2 * sin_y_2 * sin_z_2,
  }
}

pub fn from_axis_angle(axis: Vector3, angle: f32) -> Quaternion {
  let half_angle = angle / 2.0f32;
  let vector_part = axis.scale(half_angle.sin());

  Quaternion {
    x: vector_part.x,
    y: vector_part.y,
    z: vector_part.z,
    w: half_angle.cos(),
  }
}

pub fn get_axis_angle(q: Quaternion) -> (Vector3, f32) {
  let axis = q.get_vector_part().normalize();
  let angle = 2.0 * q.get_scalar_part().acos();

  if angle > ::std::f32::consts::PI {
    (axis.negate(), 2.0 * ::std::f32::consts::PI - angle)
  } else {
    (axis, angle)
  }
}

impl ops::Add<Quaternion> for Quaternion {
  type Output = Quaternion;

  fn add(self, _rhs: Quaternion) -> Self {
    Quaternion {
      x: self.x + _rhs.x,
      y: self.y + _rhs.y,
      z: self.z + _rhs.z,
      w: self.w + _rhs.w,
    }
  }
}

impl ops::Mul<Quaternion> for Quaternion {
  type Output = Quaternion;

  fn mul(self, _rhs: Quaternion) -> Self {
    let vector_a = self.get_vector_part();
    let vector_b = _rhs.get_vector_part();
    let inner_product_result = vector_a.inner_product(&vector_b);
    let vector_part =
      vector_b.scale(self.w) + vector_a.scale(_rhs.w) + vector_a.outer_product(&vector_b);

    Quaternion {
      x: vector_part.x,
      y: vector_part.y,
      z: vector_part.z,
      w: self.w * _rhs.w - inner_product_result,
    }
  }
}
