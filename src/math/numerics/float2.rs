use std::ops::*;

use crate::math::numerics::{Float3, Float4};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub const ZERO: Float2 = Float2::new(0.0, 0.0);
    pub const ONE: Float2 = Float2::new(1.0, 1.0);
    pub const TWO: Float2 = Float2::new(2.0, 2.0);
    pub const HALF: Float2 = Float2::new(0.5, 0.5);
    pub const UNIT_X: Float2 = Float2::new(1.0, 0.0);
    pub const UNIT_Y: Float2 = Float2::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Float2 {
        Float2 { x, y }
    }

    pub const fn splat(v: f32) -> Float2 {
        Float2 { x: v, y: v }
    }

    pub fn xxx(self) -> Float3 { Float3::new(self.x, self.x, self.x) }
    pub fn xxy(self) -> Float3 { Float3::new(self.x, self.x, self.y) }
    pub fn xyx(self) -> Float3 { Float3::new(self.x, self.y, self.x) }
    pub fn xyy(self) -> Float3 { Float3::new(self.x, self.y, self.y) }
    pub fn yxx(self) -> Float3 { Float3::new(self.y, self.x, self.x) }
    pub fn yxy(self) -> Float3 { Float3::new(self.y, self.x, self.y) }
    pub fn yyx(self) -> Float3 { Float3::new(self.y, self.y, self.x) }
    pub fn yyy(self) -> Float3 { Float3::new(self.y, self.y, self.y) }

    pub fn xxxx(self) -> Float4 { Float4::new(self.x, self.x, self.x, self.x) }
    pub fn xxyy(self) -> Float4 { Float4::new(self.x, self.x, self.y, self.y) }
    pub fn xyxy(self) -> Float4 { Float4::new(self.x, self.y, self.x, self.y) }
    pub fn xyyx(self) -> Float4 { Float4::new(self.x, self.y, self.y, self.x) }
    pub fn yyyy(self) -> Float4 { Float4::new(self.y, self.y, self.y, self.y) }

    #[inline(always)]
    pub fn dot(self, rhs: Float2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline(always)]
    pub fn cross(self, rhs: Float2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
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
    pub fn normalize(self) -> Float2 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    #[inline(always)]
    pub fn lerp(self, rhs: Float2, t: f32) -> Float2 {
        self + (rhs - self) * t
    }

    #[inline(always)]
    pub fn min(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x.min(rhs.x), y: self.y.min(rhs.y) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x.max(rhs.x), y: self.y.max(rhs.y) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Float2, max: Float2) -> Float2 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Float2 {
        Float2 { x: self.x.abs(), y: self.y.abs() }
    }

    #[inline(always)]
    pub fn floor(self) -> Float2 {
        Float2 { x: self.x.floor(), y: self.y.floor() }
    }

    #[inline(always)]
    pub fn ceil(self) -> Float2 {
        Float2 { x: self.x.ceil(), y: self.y.ceil() }
    }

    #[inline(always)]
    pub fn round(self) -> Float2 {
        Float2 { x: self.x.round(), y: self.y.round() }
    }

    #[inline(always)]
    pub fn fract(self) -> Float2 {
        Float2 { x: self.x.fract(), y: self.y.fract() }
    }

    #[inline(always)]
    pub fn min_component(self) -> f32 {
        self.x.min(self.y)
    }

    #[inline(always)]
    pub fn max_component(self) -> f32 {
        self.x.max(self.y)
    }
}

impl Neg for Float2 {
    type Output = Float2;
    fn neg(self) -> Float2 {
        Float2 { x: -self.x, y: -self.y }
    }
}

impl Add<Float2> for Float2 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<f32> for Float2 {
    type Output = Float2;
    fn add(self, rhs: f32) -> Float2 {
        Float2 { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<Float2> for f32 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Float2 {
        Float2 { x: self + rhs.x, y: self + rhs.y }
    }
}
impl AddAssign<Float2> for Float2 {
    fn add_assign(&mut self, rhs: Float2) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<f32> for Float2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs;
    }
}

impl Sub<Float2> for Float2 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Sub<f32> for Float2 {
    type Output = Float2;
    fn sub(self, rhs: f32) -> Float2 {
        Float2 { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<Float2> for f32 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Float2 {
        Float2 { x: self - rhs.x, y: self - rhs.y }
    }
}
impl SubAssign<Float2> for Float2 {
    fn sub_assign(&mut self, rhs: Float2) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<f32> for Float2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs;
    }
}

impl Mul<Float2> for Float2 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
impl Mul<f32> for Float2 {
    type Output = Float2;
    fn mul(self, rhs: f32) -> Float2 {
        Float2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<Float2> for f32 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Float2 {
        Float2 { x: self * rhs.x, y: self * rhs.y }
    }
}
impl MulAssign<Float2> for Float2 {
    fn mul_assign(&mut self, rhs: Float2) {
        self.x *= rhs.x; self.y *= rhs.y;
    }
}
impl MulAssign<f32> for Float2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs;
    }
}

impl Div<Float2> for Float2 {
    type Output = Float2;
    fn div(self, rhs: Float2) -> Float2 {
        Float2 { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}
impl Div<f32> for Float2 {
    type Output = Float2;
    fn div(self, rhs: f32) -> Float2 {
        Float2 { x: self.x / rhs, y: self.y / rhs }
    }
}
impl Div<Float2> for f32 {
    type Output = Float2;
    fn div(self, rhs: Float2) -> Float2 {
        Float2 { x: self / rhs.x, y: self / rhs.y }
    }
}
impl DivAssign<Float2> for Float2 {
    fn div_assign(&mut self, rhs: Float2) {
        self.x /= rhs.x; self.y /= rhs.y;
    }
}
impl DivAssign<f32> for Float2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs;
    }
}

impl From<(f32, f32)> for Float2 {
    fn from((x, y): (f32, f32)) -> Float2 {
        Float2 { x, y }
    }
}
impl From<[f32; 2]> for Float2 {
    fn from([x, y]: [f32; 2]) -> Float2 {
        Float2 { x, y }
    }
}
impl From<Float2> for [f32; 2] {
    fn from(v: Float2) -> [f32; 2] {
        [v.x, v.y]
    }
}
