use std::ops::*;

use crate::math::numerics::{Float2, Int2, UInt3, UInt4};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UInt2 {
    pub x: u32,
    pub y: u32,
}

impl UInt2 {
    pub const ZERO: UInt2 = UInt2::new(0, 0);
    pub const ONE: UInt2 = UInt2::new(1, 1);
    pub const UNIT_X: UInt2 = UInt2::new(1, 0);
    pub const UNIT_Y: UInt2 = UInt2::new(0, 1);

    pub const fn new(x: u32, y: u32) -> UInt2 {
        UInt2 { x, y }
    }

    pub const fn splat(v: u32) -> UInt2 {
        UInt2 { x: v, y: v }
    }

    pub fn xxx(self) -> UInt3 { UInt3::new(self.x, self.x, self.x) }
    pub fn xxy(self) -> UInt3 { UInt3::new(self.x, self.x, self.y) }
    pub fn xyx(self) -> UInt3 { UInt3::new(self.x, self.y, self.x) }
    pub fn xyy(self) -> UInt3 { UInt3::new(self.x, self.y, self.y) }
    pub fn yxx(self) -> UInt3 { UInt3::new(self.y, self.x, self.x) }
    pub fn yxy(self) -> UInt3 { UInt3::new(self.y, self.x, self.y) }
    pub fn yyx(self) -> UInt3 { UInt3::new(self.y, self.y, self.x) }
    pub fn yyy(self) -> UInt3 { UInt3::new(self.y, self.y, self.y) }

    pub fn xyzw(self, z: u32, w: u32) -> UInt4 { UInt4::new(self.x, self.y, z, w) }

    #[inline(always)]
    pub fn min(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x.min(rhs.x), y: self.y.min(rhs.y) }
    }

    #[inline(always)]
    pub fn max(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x.max(rhs.x), y: self.y.max(rhs.y) }
    }

    #[inline(always)]
    pub fn clamp(self, min: UInt2, max: UInt2) -> UInt2 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn dot(self, rhs: UInt2) -> u32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline(always)]
    pub fn to_int2(self) -> Int2 {
        Int2 { x: self.x as i32, y: self.y as i32 }
    }

    #[inline(always)]
    pub fn to_float2(self) -> Float2 {
        Float2 { x: self.x as f32, y: self.y as f32 }
    }
}

impl Add<UInt2> for UInt2 {
    type Output = UInt2;
    fn add(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<u32> for UInt2 {
    type Output = UInt2;
    fn add(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<UInt2> for u32 {
    type Output = UInt2;
    fn add(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self + rhs.x, y: self + rhs.y }
    }
}
impl AddAssign<UInt2> for UInt2 {
    fn add_assign(&mut self, rhs: UInt2) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<u32> for UInt2 {
    fn add_assign(&mut self, rhs: u32) {
        self.x += rhs; self.y += rhs;
    }
}

impl Sub<UInt2> for UInt2 {
    type Output = UInt2;
    fn sub(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Sub<u32> for UInt2 {
    type Output = UInt2;
    fn sub(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<UInt2> for u32 {
    type Output = UInt2;
    fn sub(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self - rhs.x, y: self - rhs.y }
    }
}
impl SubAssign<UInt2> for UInt2 {
    fn sub_assign(&mut self, rhs: UInt2) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<u32> for UInt2 {
    fn sub_assign(&mut self, rhs: u32) {
        self.x -= rhs; self.y -= rhs;
    }
}

impl Mul<UInt2> for UInt2 {
    type Output = UInt2;
    fn mul(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
impl Mul<u32> for UInt2 {
    type Output = UInt2;
    fn mul(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<UInt2> for u32 {
    type Output = UInt2;
    fn mul(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self * rhs.x, y: self * rhs.y }
    }
}
impl MulAssign<UInt2> for UInt2 {
    fn mul_assign(&mut self, rhs: UInt2) {
        self.x *= rhs.x; self.y *= rhs.y;
    }
}
impl MulAssign<u32> for UInt2 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs; self.y *= rhs;
    }
}

impl Div<UInt2> for UInt2 {
    type Output = UInt2;
    fn div(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}
impl Div<u32> for UInt2 {
    type Output = UInt2;
    fn div(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x / rhs, y: self.y / rhs }
    }
}
impl DivAssign<UInt2> for UInt2 {
    fn div_assign(&mut self, rhs: UInt2) {
        self.x /= rhs.x; self.y /= rhs.y;
    }
}
impl DivAssign<u32> for UInt2 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs; self.y /= rhs;
    }
}

impl Rem<UInt2> for UInt2 {
    type Output = UInt2;
    fn rem(self, rhs: UInt2) -> UInt2 {
        UInt2 { x: self.x % rhs.x, y: self.y % rhs.y }
    }
}
impl Rem<u32> for UInt2 {
    type Output = UInt2;
    fn rem(self, rhs: u32) -> UInt2 {
        UInt2 { x: self.x % rhs, y: self.y % rhs }
    }
}
impl RemAssign<UInt2> for UInt2 {
    fn rem_assign(&mut self, rhs: UInt2) {
        self.x %= rhs.x; self.y %= rhs.y;
    }
}
impl RemAssign<u32> for UInt2 {
    fn rem_assign(&mut self, rhs: u32) {
        self.x %= rhs; self.y %= rhs;
    }
}

impl From<(u32, u32)> for UInt2 {
    fn from((x, y): (u32, u32)) -> UInt2 {
        UInt2 { x, y }
    }
}
impl From<[u32; 2]> for UInt2 {
    fn from([x, y]: [u32; 2]) -> UInt2 {
        UInt2 { x, y }
    }
}
impl From<UInt2> for [u32; 2] {
    fn from(v: UInt2) -> [u32; 2] {
        [v.x, v.y]
    }
}
