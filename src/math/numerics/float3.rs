use std::ops::*;

use crate::math::numerics::{Float2, Float4};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    pub const ZERO: Float3 = Float3::new(0.0, 0.0, 0.0);
    pub const ONE: Float3 = Float3::new(1.0, 1.0, 1.0);
    pub const TWO: Float3 = Float3::new(2.0, 2.0, 2.0);
    pub const HALF: Float3 = Float3::new(0.5, 0.5, 0.5);
    pub const UNIT_X: Float3 = Float3::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Float3 = Float3::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Float3 = Float3::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Float3 {
        Float3 { x, y, z }
    }

    pub const fn splat(v: f32) -> Float3 {
        Float3 { x: v, y: v, z: v }
    }

    pub fn from2(xy: Float2, z: f32) -> Float3 {
        Float3 { x: xy.x, y: xy.y, z }
    }

    pub fn xx(self) -> Float2 { Float2::new(self.x, self.x) }
    pub fn xy(self) -> Float2 { Float2::new(self.x, self.y) }
    pub fn xz(self) -> Float2 { Float2::new(self.x, self.z) }
    pub fn yx(self) -> Float2 { Float2::new(self.y, self.x) }
    pub fn yy(self) -> Float2 { Float2::new(self.y, self.y) }
    pub fn yz(self) -> Float2 { Float2::new(self.y, self.z) }
    pub fn zx(self) -> Float2 { Float2::new(self.z, self.x) }
    pub fn zy(self) -> Float2 { Float2::new(self.z, self.y) }
    pub fn zz(self) -> Float2 { Float2::new(self.z, self.z) }

    pub fn xxxx(self) -> Float4 { Float4::new(self.x, self.x, self.x, self.x) }
    pub fn xyzx(self) -> Float4 { Float4::new(self.x, self.y, self.z, self.x) }
    pub fn xyzy(self) -> Float4 { Float4::new(self.x, self.y, self.z, self.y) }
    pub fn xyzz(self) -> Float4 { Float4::new(self.x, self.y, self.z, self.z) }
    pub fn xyzw(self, w: f32) -> Float4 { Float4::new(self.x, self.y, self.z, w) }

    #[inline(always)]
    pub fn dot(self, rhs: Float3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    pub fn cross(self, rhs: Float3) -> Float3 {
        Float3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline(always)]
    pub fn determinant(self, b: Float3, c: Float3) -> f32 {
        self.dot(b.cross(c))
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Float3 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    #[inline(always)]
    pub fn lerp(self, rhs: Float3, t: f32) -> Float3 {
        self + (rhs - self) * t
    }

    #[inline(always)]
    pub fn reflect(self, normal: Float3) -> Float3 {
        self - normal * (2.0 * self.dot(normal))
    }

    #[inline(always)]
    pub fn min(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Float3, max: Float3) -> Float3 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Float3 {
        Float3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    #[inline(always)]
    pub fn floor(self) -> Float3 {
        Float3 { x: self.x.floor(), y: self.y.floor(), z: self.z.floor() }
    }

    #[inline(always)]
    pub fn ceil(self) -> Float3 {
        Float3 { x: self.x.ceil(), y: self.y.ceil(), z: self.z.ceil() }
    }

    #[inline(always)]
    pub fn round(self) -> Float3 {
        Float3 { x: self.x.round(), y: self.y.round(), z: self.z.round() }
    }

    #[inline(always)]
    pub fn fract(self) -> Float3 {
        Float3 { x: self.x.fract(), y: self.y.fract(), z: self.z.fract() }
    }

    #[inline(always)]
    pub fn min_component(self) -> f32 {
        self.x.min(self.y).min(self.z)
    }

    #[inline(always)]
    pub fn max_component(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }
}

impl Neg for Float3 {
    type Output = Float3;
    fn neg(self) -> Float3 {
        Float3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add<Float3> for Float3 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<f32> for Float3 {
    type Output = Float3;
    fn add(self, rhs: f32) -> Float3 {
        Float3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<Float3> for f32 {
    type Output = Float3;
    fn add(self, rhs: Float3) -> Float3 {
        Float3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}
impl AddAssign<Float3> for Float3 {
    fn add_assign(&mut self, rhs: Float3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<f32> for Float3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

impl Sub<Float3> for Float3 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<f32> for Float3 {
    type Output = Float3;
    fn sub(self, rhs: f32) -> Float3 {
        Float3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<Float3> for f32 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Float3 {
        Float3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}
impl SubAssign<Float3> for Float3 {
    fn sub_assign(&mut self, rhs: Float3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<f32> for Float3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

impl Mul<Float3> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<f32> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: f32) -> Float3 {
        Float3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Float3> for f32 {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Float3 {
        Float3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}
impl MulAssign<Float3> for Float3 {
    fn mul_assign(&mut self, rhs: Float3) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z;
    }
}
impl MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

impl Div<Float3> for Float3 {
    type Output = Float3;
    fn div(self, rhs: Float3) -> Float3 {
        Float3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<f32> for Float3 {
    type Output = Float3;
    fn div(self, rhs: f32) -> Float3 {
        Float3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl Div<Float3> for f32 {
    type Output = Float3;
    fn div(self, rhs: Float3) -> Float3 {
        Float3 { x: self / rhs.x, y: self / rhs.y, z: self / rhs.z }
    }
}
impl DivAssign<Float3> for Float3 {
    fn div_assign(&mut self, rhs: Float3) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z;
    }
}
impl DivAssign<f32> for Float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}

impl From<(f32, f32, f32)> for Float3 {
    fn from((x, y, z): (f32, f32, f32)) -> Float3 {
        Float3 { x, y, z }
    }
}
impl From<[f32; 3]> for Float3 {
    fn from([x, y, z]: [f32; 3]) -> Float3 {
        Float3 { x, y, z }
    }
}
impl From<Float3> for [f32; 3] {
    fn from(v: Float3) -> [f32; 3] {
        [v.x, v.y, v.z]
    }
}
