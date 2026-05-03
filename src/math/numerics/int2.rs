use std::ops::*;

use crate::math::numerics::{Float2, Int3, Int4, UInt2};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

impl Int2 {
    pub const ZERO: Int2 = Int2::new(0, 0);
    pub const ONE: Int2 = Int2::new(1, 1);
    pub const UNIT_X: Int2 = Int2::new(1, 0);
    pub const UNIT_Y: Int2 = Int2::new(0, 1);

    pub const fn new(x: i32, y: i32) -> Int2 {
        Int2 { x, y }
    }

    pub const fn splat(v: i32) -> Int2 {
        Int2 { x: v, y: v }
    }

    pub fn xxx(self) -> Int3 { Int3::new(self.x, self.x, self.x) }
    pub fn xxy(self) -> Int3 { Int3::new(self.x, self.x, self.y) }
    pub fn xyx(self) -> Int3 { Int3::new(self.x, self.y, self.x) }
    pub fn xyy(self) -> Int3 { Int3::new(self.x, self.y, self.y) }
    pub fn yxx(self) -> Int3 { Int3::new(self.y, self.x, self.x) }
    pub fn yxy(self) -> Int3 { Int3::new(self.y, self.x, self.y) }
    pub fn yyx(self) -> Int3 { Int3::new(self.y, self.y, self.x) }
    pub fn yyy(self) -> Int3 { Int3::new(self.y, self.y, self.y) }

    pub fn xyzw(self, z: i32, w: i32) -> Int4 { Int4::new(self.x, self.y, z, w) }

    #[inline(always)]
    pub fn min(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x.min(rhs.x), y: self.y.min(rhs.y) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x.max(rhs.x), y: self.y.max(rhs.y) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Int2, max: Int2) -> Int2 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Int2 {
        Int2 { x: self.x.abs(), y: self.y.abs() }
    }

    #[inline(always)]
    pub fn signum(self) -> Int2 {
        Int2 { x: self.x.signum(), y: self.y.signum() }
    }

    #[inline(always)]
    pub fn dot(self, rhs: Int2) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline(always)]
    pub fn div_euclid(self, rhs: i32) -> Int2 {
        Int2 { x: self.x.div_euclid(rhs), y: self.y.div_euclid(rhs) }
    }

    #[inline(always)]
    pub fn rem_euclid(self, rhs: i32) -> Int2 {
        Int2 { x: self.x.rem_euclid(rhs), y: self.y.rem_euclid(rhs) }
    }

    #[inline(always)]
    pub fn to_uint2(self) -> UInt2 {
        UInt2 { x: self.x as u32, y: self.y as u32 }
    }

    #[inline(always)]
    pub fn to_float2(self) -> Float2 {
        Float2 { x: self.x as f32, y: self.y as f32 }
    }
}

impl Neg for Int2 {
    type Output = Int2;
    fn neg(self) -> Int2 {
        Int2 { x: -self.x, y: -self.y }
    }
}

impl Add<Int2> for Int2 {
    type Output = Int2;
    fn add(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<i32> for Int2 {
    type Output = Int2;
    fn add(self, rhs: i32) -> Int2 {
        Int2 { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<Int2> for i32 {
    type Output = Int2;
    fn add(self, rhs: Int2) -> Int2 {
        Int2 { x: self + rhs.x, y: self + rhs.y }
    }
}
impl AddAssign<Int2> for Int2 {
    fn add_assign(&mut self, rhs: Int2) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<i32> for Int2 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; self.y += rhs;
    }
}

impl Sub<Int2> for Int2 {
    type Output = Int2;
    fn sub(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Sub<i32> for Int2 {
    type Output = Int2;
    fn sub(self, rhs: i32) -> Int2 {
        Int2 { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<Int2> for i32 {
    type Output = Int2;
    fn sub(self, rhs: Int2) -> Int2 {
        Int2 { x: self - rhs.x, y: self - rhs.y }
    }
}
impl SubAssign<Int2> for Int2 {
    fn sub_assign(&mut self, rhs: Int2) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<i32> for Int2 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; self.y -= rhs;
    }
}

impl Mul<Int2> for Int2 {
    type Output = Int2;
    fn mul(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
impl Mul<i32> for Int2 {
    type Output = Int2;
    fn mul(self, rhs: i32) -> Int2 {
        Int2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<Int2> for i32 {
    type Output = Int2;
    fn mul(self, rhs: Int2) -> Int2 {
        Int2 { x: self * rhs.x, y: self * rhs.y }
    }
}
impl MulAssign<Int2> for Int2 {
    fn mul_assign(&mut self, rhs: Int2) {
        self.x *= rhs.x; self.y *= rhs.y;
    }
}
impl MulAssign<i32> for Int2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; self.y *= rhs;
    }
}

impl Div<Int2> for Int2 {
    type Output = Int2;
    fn div(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}
impl Div<i32> for Int2 {
    type Output = Int2;
    fn div(self, rhs: i32) -> Int2 {
        Int2 { x: self.x / rhs, y: self.y / rhs }
    }
}
impl DivAssign<Int2> for Int2 {
    fn div_assign(&mut self, rhs: Int2) {
        self.x /= rhs.x; self.y /= rhs.y;
    }
}
impl DivAssign<i32> for Int2 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; self.y /= rhs;
    }
}

impl Rem<Int2> for Int2 {
    type Output = Int2;
    fn rem(self, rhs: Int2) -> Int2 {
        Int2 { x: self.x % rhs.x, y: self.y % rhs.y }
    }
}
impl Rem<i32> for Int2 {
    type Output = Int2;
    fn rem(self, rhs: i32) -> Int2 {
        Int2 { x: self.x % rhs, y: self.y % rhs }
    }
}
impl RemAssign<Int2> for Int2 {
    fn rem_assign(&mut self, rhs: Int2) {
        self.x %= rhs.x; self.y %= rhs.y;
    }
}
impl RemAssign<i32> for Int2 {
    fn rem_assign(&mut self, rhs: i32) {
        self.x %= rhs; self.y %= rhs;
    }
}

impl From<(i32, i32)> for Int2 {
    fn from((x, y): (i32, i32)) -> Int2 {
        Int2 { x, y }
    }
}
impl From<[i32; 2]> for Int2 {
    fn from([x, y]: [i32; 2]) -> Int2 {
        Int2 { x, y }
    }
}
impl From<Int2> for [i32; 2] {
    fn from(v: Int2) -> [i32; 2] {
        [v.x, v.y]
    }
}
