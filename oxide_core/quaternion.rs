use crate::vector::vector3::*;
use crate::vector::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Quaternion {
  pub w: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

trait VectorPart {
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

trait ScalarPart {
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
    w: one,
    x: zero,
    y: zero,
    z: zero,
  }
}

pub fn add(a: Quaternion, b: Quaternion) -> Quaternion {
  Quaternion {
    w: a.w + b.w,
    x: a.x + b.x,
    y: a.y + b.y,
    z: a.z + b.z,
  }
}

pub fn scale(q: Quaternion, t: f32) -> Quaternion {
  Quaternion {
    w: q.w * t,
    x: q.x * t,
    y: q.y * t,
    z: q.z * t,
  }
}

pub fn mul(a: Quaternion, b: Quaternion) -> Quaternion {
  let vector_a = a.get_vector_part();
  let vector_b = b.get_vector_part();

  let inner_product_result = vector_a.inner_product(&vector_b);
  let vector_part = vector_b
    .scale(a.w) + vector_a.scale(b.w) + 
    vector_a.outer_product(&vector_b)

  Quaternion {
    w: a.w * b.w - inner_product_result,
    x: vector_part.x,
    y: vector_part.y,
    z: vector_part.z,
  }
}

pub fn conj(a: Quaternion) -> Quaternion {
  let vector_neg = a.get_vector_part().neg();

  Quaternion {
    w: a.w,
    x: vector_neg.x,
    y: vector_neg.y,
    z: vector_neg.z,
  }
}

pub fn square_len(q: Quaternion) -> f32 {
  q.w * q.w + q.get_vector_part().get_squared_length()
}

pub fn len(q: Quaternion) -> f32 {
  square_len(q).sqrt()
}

pub fn rotate_vector(q: Quaternion, v: Vector3) -> Vector3 {
  let vector_q = q.get_vector_part();
  let t: Vector3 = vector_q.outer_product(&v).scale(2.0f32);
  v + t.scale(q.w) + vector_q.outer_product(&t)
}

pub fn euler_angles(x: f32, y: f32, z: f32) -> Quaternion {
  let half_x = x / 2.0f32;
  let half_y = y / 2.0f32;
  let half_z = z / 2.0f32;

  let cos_x_2 = half_x.cos();
  let cos_y_2 = half_y.cos();
  let cos_z_2 = half_z.cos();

  let sin_x_2 = half_x.sin();
  let sin_y_2 = half_y.sin();
  let sin_z_2 = half_z.sin();

  let vector_part = Vector3 {
    x: sin_x_2 * cos_y_2 * cos_z_2 + cos_x_2 * sin_y_2 * sin_z_2,
    y: cos_x_2 * sin_y_2 * cos_z_2 + sin_x_2 * cos_y_2 * sin_z_2,
    z: cos_x_2 * cos_y_2 * sin_z_2 + sin_x_2 * sin_y_2 * cos_z_2,
  };

  Quaternion {
    w: cos_x_2 * cos_y_2 * cos_z_2 + sin_x_2 * sin_y_2 * sin_z_2,
    x: vector_part.x,
    y: vector_part.y,
    z: vector_part.z,
  }
}

pub fn axis_angle(axis: Vector3, angle: f32) -> Quaternion {
  let half_angle = angle / 2.0f32;
  let vector_part = axis.scale(half_angle.sin());

  Quaternion {
    w: half_angle.cos(),
    x: vector_part.x,
    y: vector_part.y,
    z: vector_part.z,
  }
}