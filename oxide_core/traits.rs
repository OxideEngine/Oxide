use std::ops::*;

pub trait Float:
  'static
  + Send
  + Sync
  + Copy
  + PartialEq
  + PartialOrd
  + One
  + Zero
  + Min
  + Max
  + Powf
  + Radians
  + From
  + Trig
  + Add<Self, Output = Self>
  + AddAssign<Self>
  + Mul<Self, Output = Self>
  + MulAssign<Self>
  + Sub<Self, Output = Self>
  + SubAssign<Self>
  + Div<Self, Output = Self>
  + DivAssign<Self>
  + Rem<Self, Output = Self>
  + RemAssign<Self>
  + Neg<Output = Self>
{
}

impl<T> Float for T where
  T: 'static
    + Send
    + Sync
    + Copy
    + PartialEq
    + PartialOrd
    + One
    + Zero
    + Min
    + Max
    + Powf
    + Radians
    + From
    + Trig
    + Add<T, Output = T>
    + AddAssign<T>
    + Mul<T, Output = T>
    + MulAssign<T>
    + Sub<T, Output = T>
    + SubAssign<T>
    + Div<T, Output = T>
    + DivAssign<T>
    + Rem<T, Output = T>
    + RemAssign<T>
    + Neg<Output = T>
{
}

pub trait Zero {
  fn zero() -> Self;
}

impl Zero for f32 {
  fn zero() -> Self {
    0.0f32
  }
}

impl Zero for f64 {
  fn zero() -> Self {
    0.0f64
  }
}

pub trait One {
  fn one() -> Self;
}

impl One for f32 {
  fn one() -> Self {
    1.0f32
  }
}

impl One for f64 {
  fn one() -> Self {
    1.0f64
  }
}

pub trait Sqrt {
  fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
  fn sqrt(self) -> Self {
    self.sqrt()
  }
}

impl Sqrt for f64 {
  fn sqrt(self) -> Self {
    self.sqrt()
  }
}

pub trait Min {
  fn min(self, other: Self) -> Self;
}

impl Min for f32 {
  fn min(self, other: Self) -> Self {
    self.min(other)
  }
}

impl Min for f64 {
  fn min(self, other: Self) -> Self {
    self.min(other)
  }
}

pub trait Max {
  fn max(self, other: Self) -> Self;
}

impl Max for f32 {
  fn max(self, other: Self) -> Self {
    self.max(other)
  }
}

impl Max for f64 {
  fn max(self, other: Self) -> Self {
    self.max(other)
  }
}

pub trait Powf {
  fn powf(self, other: Self) -> Self;
}

impl Powf for f32 {
  fn powf(self, other: Self) -> Self {
    self.powf(other)
  }
}

impl Powf for f64 {
  fn powf(self, other: Self) -> Self {
    self.powf(other)
  }
}

pub trait Radians {
  fn _90() -> Self;
  fn _180() -> Self;
  fn _270() -> Self;
  fn _360() -> Self;

  fn deg_to_rad(self) -> Self;
  fn rad_to_deg(self) -> Self;
}

impl Radians for f32 {
  fn _90() -> f32 {
    ::std::f32::consts::FRAC_PI_2
  }

  fn _180() -> f32 {
    ::std::f32::consts::PI
  }

  fn _270() -> f32 {
    <Self as Radians>::_90() * 3.0f32
  }

  fn _360() -> f32 {
    <Self as Radians>::_180() * 2.0f32
  }

  fn deg_to_rad(self) -> Self {
    self * (::std::f32::consts::PI / 180.0f32)
  }

  fn rad_to_deg(self) -> Self {
    self * (180.0f32 / ::std::f32::consts::PI)
  }
}

impl Radians for f64 {
  fn _90() -> f64 {
    ::std::f64::consts::FRAC_PI_2
  }

  fn _180() -> f64 {
    ::std::f64::consts::PI
  }

  fn _270() -> f64 {
    ::std::f64::consts::FRAC_PI_2 * 3.0f64
  }

  fn _360() -> f64 {
    ::std::f64::consts::PI * 2.0f64
  }

  fn deg_to_rad(self) -> Self {
    self * (::std::f64::consts::PI / 180.0f64)
  }

  fn rad_to_deg(self) -> Self {
    self * (180.0f64 / ::std::f64::consts::PI)
  }
}

pub trait Trig {
  fn sin(self) -> Self;

  fn cos(self) -> Self;

  fn tan(self) -> Self;

  fn asin(self) -> Self;

  fn acos(self) -> Self;

  fn atan(self) -> Self;

  fn atan2(self, other: Self) -> Self;

  fn sinh(self) -> Self;

  fn cosh(self) -> Self;

  fn tanh(self) -> Self;

  fn asinh(self) -> Self;

  fn acosh(self) -> Self;

  fn atanh(self) -> Self;
}

impl Trig for f32 {
  fn sin(self) -> Self {
    self.sin()
  }

  fn cos(self) -> Self {
    self.cos()
  }

  fn tan(self) -> Self {
    self.tan()
  }

  fn asin(self) -> Self {
    self.asin()
  }

  fn acos(self) -> Self {
    self.acos()
  }

  fn atan(self) -> Self {
    self.atan()
  }

  fn atan2(self, other: Self) -> Self {
    self.atan2(other)
  }

  fn sinh(self) -> Self {
    self.sinh()
  }

  fn cosh(self) -> Self {
    self.cosh()
  }

  fn tanh(self) -> Self {
    self.tanh()
  }

  fn asinh(self) -> Self {
    self.asinh()
  }

  fn acosh(self) -> Self {
    self.acosh()
  }

  fn atanh(self) -> Self {
    self.atanh()
  }
}

impl Trig for f64 {
  fn sin(self) -> Self {
    self.sin()
  }

  fn cos(self) -> Self {
    self.cos()
  }

  fn tan(self) -> Self {
    self.tan()
  }

  fn asin(self) -> Self {
    self.asin()
  }

  fn acos(self) -> Self {
    self.acos()
  }

  fn atan(self) -> Self {
    self.atan()
  }

  fn atan2(self, other: Self) -> Self {
    self.atan2(other)
  }

  fn sinh(self) -> Self {
    self.sinh()
  }

  fn cosh(self) -> Self {
    self.cosh()
  }

  fn tanh(self) -> Self {
    self.tanh()
  }

  fn asinh(self) -> Self {
    self.asinh()
  }

  fn acosh(self) -> Self {
    self.acosh()
  }

  fn atanh(self) -> Self {
    self.atanh()
  }
}

pub trait Cast<T> {
  fn cast(self) -> T;
}

impl Cast<f64> for f32 {
  fn cast(self) -> f64 {
    self as f64
  }
}

impl Cast<f32> for f64 {
  fn cast(self) -> f32 {
    self as f32
  }
}

impl Cast<f32> for f32 {
  fn cast(self) -> f32 {
    self
  }
}

impl Cast<f64> for f64 {
  fn cast(self) -> f64 {
    self
  }
}

pub trait From {
  fn from_f32(n: f32) -> Self;

  fn from_f64(n: f64) -> Self;

  fn from_isize(n: isize) -> Self;

  fn from_i32(n: i32) -> Self;

  fn from_u32(n: u32) -> Self;
}

impl From for f32 {
  fn from_f32(n: f32) -> Self {
    n
  }

  fn from_f64(n: f64) -> Self {
    n as f32
  }

  fn from_isize(n: isize) -> Self {
    n as f32
  }

  fn from_i32(n: i32) -> Self {
    n as f32
  }

  fn from_u32(n: u32) -> Self {
    n as f32
  }
}

impl From for f64 {
  fn from_f32(n: f32) -> Self {
    n as f64
  }

  fn from_f64(n: f64) -> Self {
    n
  }

  fn from_isize(n: isize) -> Self {
    n as f64
  }

  fn from_i32(n: i32) -> Self {
    n as f64
  }

  fn from_u32(n: u32) -> Self {
    n as f64
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_f32_sqrt() {
    let a = 3.0f32;
    let b = <f32 as Sqrt>::sqrt(9.0f32);
    assert_eq!(b, a);
  }

  #[test]
  fn test_f32_square() {
    let a = <f32 as Powf>::powf(3f32, 2f32);
    let b = 9f32;
    assert_eq!(a, b);
  }

  #[test]
  fn test_f32_deg_to_rad() {
    let degree = 23.0f32;
    let radian = degree.deg_to_rad();
    assert!((radian - 0.401425).abs() > ::std::f32::EPSILON);
  }

  #[test]
  fn test_f64_deg_to_rad() {
    let degree = 60.0f64;
    let radian = degree.deg_to_rad();
    assert!((radian - 1.047197).abs() > ::std::f64::EPSILON);
  }
}
