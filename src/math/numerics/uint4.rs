use std::ops::*;

use crate::math::numerics::{Float4, Int4, UInt2, UInt3};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UInt4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

impl UInt4 {
    pub const ZERO: UInt4 = UInt4::new(0, 0, 0, 0);
    pub const ONE: UInt4 = UInt4::new(1, 1, 1, 1);
    pub const UNIT_X: UInt4 = UInt4::new(1, 0, 0, 0);
    pub const UNIT_Y: UInt4 = UInt4::new(0, 1, 0, 0);
    pub const UNIT_Z: UInt4 = UInt4::new(0, 0, 1, 0);
    pub const UNIT_W: UInt4 = UInt4::new(0, 0, 0, 1);

    pub const fn new(x: u32, y: u32, z: u32, w: u32) -> UInt4 {
        UInt4 { x, y, z, w }
    }

    pub const fn splat(v: u32) -> UInt4 {
        UInt4 { x: v, y: v, z: v, w: v }
    }

    pub fn from2(xy: UInt2, z: u32, w: u32) -> UInt4 {
        UInt4 { x: xy.x, y: xy.y, z, w }
    }

    pub fn from3(xyz: UInt3, w: u32) -> UInt4 {
        UInt4 { x: xyz.x, y: xyz.y, z: xyz.z, w }
    }

    pub fn xx(self) -> UInt2 { UInt2::new(self.x, self.x) }
    pub fn xy(self) -> UInt2 { UInt2::new(self.x, self.y) }
    pub fn xz(self) -> UInt2 { UInt2::new(self.x, self.z) }
    pub fn xw(self) -> UInt2 { UInt2::new(self.x, self.w) }
    pub fn yx(self) -> UInt2 { UInt2::new(self.y, self.x) }
    pub fn yy(self) -> UInt2 { UInt2::new(self.y, self.y) }
    pub fn yz(self) -> UInt2 { UInt2::new(self.y, self.z) }
    pub fn yw(self) -> UInt2 { UInt2::new(self.y, self.w) }
    pub fn zx(self) -> UInt2 { UInt2::new(self.z, self.x) }
    pub fn zy(self) -> UInt2 { UInt2::new(self.z, self.y) }
    pub fn zz(self) -> UInt2 { UInt2::new(self.z, self.z) }
    pub fn zw(self) -> UInt2 { UInt2::new(self.z, self.w) }
    pub fn wx(self) -> UInt2 { UInt2::new(self.w, self.x) }
    pub fn wy(self) -> UInt2 { UInt2::new(self.w, self.y) }
    pub fn wz(self) -> UInt2 { UInt2::new(self.w, self.z) }
    pub fn ww(self) -> UInt2 { UInt2::new(self.w, self.w) }

    pub fn xyz(self) -> UInt3 { UInt3::new(self.x, self.y, self.z) }
    pub fn xyw(self) -> UInt3 { UInt3::new(self.x, self.y, self.w) }
    pub fn xzw(self) -> UInt3 { UInt3::new(self.x, self.z, self.w) }
    pub fn yzw(self) -> UInt3 { UInt3::new(self.y, self.z, self.w) }

    #[inline(always)]
    pub fn min(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z), w: self.w.min(rhs.w) }
    }

    #[inline(always)]
    pub fn max(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z), w: self.w.max(rhs.w) }
    }

    #[inline(always)]
    pub fn clamp(self, min: UInt4, max: UInt4) -> UInt4 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn dot(self, rhs: UInt4) -> u32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline(always)]
    pub fn to_int4(self) -> Int4 {
        Int4 { x: self.x as i32, y: self.y as i32, z: self.z as i32, w: self.w as i32 }
    }

    #[inline(always)]
    pub fn to_float4(self) -> Float4 {
        Float4 { x: self.x as f32, y: self.y as f32, z: self.z as f32, w: self.w as f32 }
    }
}

impl Add<UInt4> for UInt4 {
    type Output = UInt4;
    fn add(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}
impl Add<u32> for UInt4 {
    type Output = UInt4;
    fn add(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs }
    }
}
impl Add<UInt4> for u32 {
    type Output = UInt4;
    fn add(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z, w: self + rhs.w }
    }
}
impl AddAssign<UInt4> for UInt4 {
    fn add_assign(&mut self, rhs: UInt4) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w;
    }
}
impl AddAssign<u32> for UInt4 {
    fn add_assign(&mut self, rhs: u32) {
        self.x += rhs; self.y += rhs; self.z += rhs; self.w += rhs;
    }
}

impl Sub<UInt4> for UInt4 {
    type Output = UInt4;
    fn sub(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}
impl Sub<u32> for UInt4 {
    type Output = UInt4;
    fn sub(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}
impl Sub<UInt4> for u32 {
    type Output = UInt4;
    fn sub(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z, w: self - rhs.w }
    }
}
impl SubAssign<UInt4> for UInt4 {
    fn sub_assign(&mut self, rhs: UInt4) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; self.w -= rhs.w;
    }
}
impl SubAssign<u32> for UInt4 {
    fn sub_assign(&mut self, rhs: u32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs; self.w -= rhs;
    }
}

impl Mul<UInt4> for UInt4 {
    type Output = UInt4;
    fn mul(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}
impl Mul<u32> for UInt4 {
    type Output = UInt4;
    fn mul(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}
impl Mul<UInt4> for u32 {
    type Output = UInt4;
    fn mul(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}
impl MulAssign<UInt4> for UInt4 {
    fn mul_assign(&mut self, rhs: UInt4) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z; self.w *= rhs.w;
    }
}
impl MulAssign<u32> for UInt4 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs; self.w *= rhs;
    }
}

impl Div<UInt4> for UInt4 {
    type Output = UInt4;
    fn div(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}
impl Div<u32> for UInt4 {
    type Output = UInt4;
    fn div(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}
impl DivAssign<UInt4> for UInt4 {
    fn div_assign(&mut self, rhs: UInt4) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z; self.w /= rhs.w;
    }
}
impl DivAssign<u32> for UInt4 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs; self.w /= rhs;
    }
}

impl Rem<UInt4> for UInt4 {
    type Output = UInt4;
    fn rem(self, rhs: UInt4) -> UInt4 {
        UInt4 { x: self.x % rhs.x, y: self.y % rhs.y, z: self.z % rhs.z, w: self.w % rhs.w }
    }
}
impl Rem<u32> for UInt4 {
    type Output = UInt4;
    fn rem(self, rhs: u32) -> UInt4 {
        UInt4 { x: self.x % rhs, y: self.y % rhs, z: self.z % rhs, w: self.w % rhs }
    }
}
impl RemAssign<UInt4> for UInt4 {
    fn rem_assign(&mut self, rhs: UInt4) {
        self.x %= rhs.x; self.y %= rhs.y; self.z %= rhs.z; self.w %= rhs.w;
    }
}
impl RemAssign<u32> for UInt4 {
    fn rem_assign(&mut self, rhs: u32) {
        self.x %= rhs; self.y %= rhs; self.z %= rhs; self.w %= rhs;
    }
}

impl From<(u32, u32, u32, u32)> for UInt4 {
    fn from((x, y, z, w): (u32, u32, u32, u32)) -> UInt4 {
        UInt4 { x, y, z, w }
    }
}
impl From<[u32; 4]> for UInt4 {
    fn from([x, y, z, w]: [u32; 4]) -> UInt4 {
        UInt4 { x, y, z, w }
    }
}
impl From<UInt4> for [u32; 4] {
    fn from(v: UInt4) -> [u32; 4] {
        [v.x, v.y, v.z, v.w]
    }
}
