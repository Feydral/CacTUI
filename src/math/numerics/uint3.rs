use std::ops::*;

use crate::math::numerics::{Float3, Int3, UInt2, UInt4};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UInt3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl UInt3 {
    pub const ZERO: UInt3 = UInt3::new(0, 0, 0);
    pub const ONE: UInt3 = UInt3::new(1, 1, 1);
    pub const UNIT_X: UInt3 = UInt3::new(1, 0, 0);
    pub const UNIT_Y: UInt3 = UInt3::new(0, 1, 0);
    pub const UNIT_Z: UInt3 = UInt3::new(0, 0, 1);

    pub const fn new(x: u32, y: u32, z: u32) -> UInt3 {
        UInt3 { x, y, z }
    }

    pub const fn splat(v: u32) -> UInt3 {
        UInt3 { x: v, y: v, z: v }
    }

    pub fn from2(xy: UInt2, z: u32) -> UInt3 {
        UInt3 { x: xy.x, y: xy.y, z }
    }

    pub fn xx(self) -> UInt2 { UInt2::new(self.x, self.x) }
    pub fn xy(self) -> UInt2 { UInt2::new(self.x, self.y) }
    pub fn xz(self) -> UInt2 { UInt2::new(self.x, self.z) }
    pub fn yx(self) -> UInt2 { UInt2::new(self.y, self.x) }
    pub fn yy(self) -> UInt2 { UInt2::new(self.y, self.y) }
    pub fn yz(self) -> UInt2 { UInt2::new(self.y, self.z) }
    pub fn zx(self) -> UInt2 { UInt2::new(self.z, self.x) }
    pub fn zy(self) -> UInt2 { UInt2::new(self.z, self.y) }
    pub fn zz(self) -> UInt2 { UInt2::new(self.z, self.z) }

    pub fn xyzw(self, w: u32) -> UInt4 { UInt4::new(self.x, self.y, self.z, w) }

    #[inline(always)]
    pub fn min(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z) }
    }

    #[inline(always)]
    pub fn max(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z) }
    }

    #[inline(always)]
    pub fn clamp(self, min: UInt3, max: UInt3) -> UInt3 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn dot(self, rhs: UInt3) -> u32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    pub fn to_int3(self) -> Int3 {
        Int3 { x: self.x as i32, y: self.y as i32, z: self.z as i32 }
    }

    #[inline(always)]
    pub fn to_float3(self) -> Float3 {
        Float3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

impl Add<UInt3> for UInt3 {
    type Output = UInt3;
    fn add(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<u32> for UInt3 {
    type Output = UInt3;
    fn add(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<UInt3> for u32 {
    type Output = UInt3;
    fn add(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}
impl AddAssign<UInt3> for UInt3 {
    fn add_assign(&mut self, rhs: UInt3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<u32> for UInt3 {
    fn add_assign(&mut self, rhs: u32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

impl Sub<UInt3> for UInt3 {
    type Output = UInt3;
    fn sub(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<u32> for UInt3 {
    type Output = UInt3;
    fn sub(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<UInt3> for u32 {
    type Output = UInt3;
    fn sub(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}
impl SubAssign<UInt3> for UInt3 {
    fn sub_assign(&mut self, rhs: UInt3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<u32> for UInt3 {
    fn sub_assign(&mut self, rhs: u32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

impl Mul<UInt3> for UInt3 {
    type Output = UInt3;
    fn mul(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<u32> for UInt3 {
    type Output = UInt3;
    fn mul(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<UInt3> for u32 {
    type Output = UInt3;
    fn mul(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}
impl MulAssign<UInt3> for UInt3 {
    fn mul_assign(&mut self, rhs: UInt3) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z;
    }
}
impl MulAssign<u32> for UInt3 {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

impl Div<UInt3> for UInt3 {
    type Output = UInt3;
    fn div(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<u32> for UInt3 {
    type Output = UInt3;
    fn div(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl DivAssign<UInt3> for UInt3 {
    fn div_assign(&mut self, rhs: UInt3) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z;
    }
}
impl DivAssign<u32> for UInt3 {
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}

impl Rem<UInt3> for UInt3 {
    type Output = UInt3;
    fn rem(self, rhs: UInt3) -> UInt3 {
        UInt3 { x: self.x % rhs.x, y: self.y % rhs.y, z: self.z % rhs.z }
    }
}
impl Rem<u32> for UInt3 {
    type Output = UInt3;
    fn rem(self, rhs: u32) -> UInt3 {
        UInt3 { x: self.x % rhs, y: self.y % rhs, z: self.z % rhs }
    }
}
impl RemAssign<UInt3> for UInt3 {
    fn rem_assign(&mut self, rhs: UInt3) {
        self.x %= rhs.x; self.y %= rhs.y; self.z %= rhs.z;
    }
}
impl RemAssign<u32> for UInt3 {
    fn rem_assign(&mut self, rhs: u32) {
        self.x %= rhs; self.y %= rhs; self.z %= rhs;
    }
}

impl From<(u32, u32, u32)> for UInt3 {
    fn from((x, y, z): (u32, u32, u32)) -> UInt3 {
        UInt3 { x, y, z }
    }
}
impl From<[u32; 3]> for UInt3 {
    fn from([x, y, z]: [u32; 3]) -> UInt3 {
        UInt3 { x, y, z }
    }
}
impl From<UInt3> for [u32; 3] {
    fn from(v: UInt3) -> [u32; 3] {
        [v.x, v.y, v.z]
    }
}
