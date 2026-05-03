use std::ops::*;

use crate::math::numerics::{Float3, Int2, Int4, UInt3};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Int3 {
    pub const ZERO: Int3 = Int3::new(0, 0, 0);
    pub const ONE: Int3 = Int3::new(1, 1, 1);
    pub const UNIT_X: Int3 = Int3::new(1, 0, 0);
    pub const UNIT_Y: Int3 = Int3::new(0, 1, 0);
    pub const UNIT_Z: Int3 = Int3::new(0, 0, 1);

    pub const fn new(x: i32, y: i32, z: i32) -> Int3 {
        Int3 { x, y, z }
    }

    pub const fn splat(v: i32) -> Int3 {
        Int3 { x: v, y: v, z: v }
    }

    pub fn from2(xy: Int2, z: i32) -> Int3 {
        Int3 { x: xy.x, y: xy.y, z }
    }

    pub fn xx(self) -> Int2 { Int2::new(self.x, self.x) }
    pub fn xy(self) -> Int2 { Int2::new(self.x, self.y) }
    pub fn xz(self) -> Int2 { Int2::new(self.x, self.z) }
    pub fn yx(self) -> Int2 { Int2::new(self.y, self.x) }
    pub fn yy(self) -> Int2 { Int2::new(self.y, self.y) }
    pub fn yz(self) -> Int2 { Int2::new(self.y, self.z) }
    pub fn zx(self) -> Int2 { Int2::new(self.z, self.x) }
    pub fn zy(self) -> Int2 { Int2::new(self.z, self.y) }
    pub fn zz(self) -> Int2 { Int2::new(self.z, self.z) }

    pub fn xyzw(self, w: i32) -> Int4 { Int4::new(self.x, self.y, self.z, w) }

    #[inline(always)]
    pub fn min(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Int3, max: Int3) -> Int3 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Int3 {
        Int3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    #[inline(always)]
    pub fn signum(self) -> Int3 {
        Int3 { x: self.x.signum(), y: self.y.signum(), z: self.z.signum() }
    }

    #[inline(always)]
    pub fn dot(self, rhs: Int3) -> i32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    pub fn div_euclid(self, rhs: i32) -> Int3 {
        Int3 { x: self.x.div_euclid(rhs), y: self.y.div_euclid(rhs), z: self.z.div_euclid(rhs) }
    }

    #[inline(always)]
    pub fn rem_euclid(self, rhs: i32) -> Int3 {
        Int3 { x: self.x.rem_euclid(rhs), y: self.y.rem_euclid(rhs), z: self.z.rem_euclid(rhs) }
    }

    #[inline(always)]
    pub fn to_uint3(self) -> UInt3 {
        UInt3 { x: self.x as u32, y: self.y as u32, z: self.z as u32 }
    }

    #[inline(always)]
    pub fn to_float3(self) -> Float3 {
        Float3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

impl Neg for Int3 {
    type Output = Int3;
    fn neg(self) -> Int3 {
        Int3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add<Int3> for Int3 {
    type Output = Int3;
    fn add(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add<i32> for Int3 {
    type Output = Int3;
    fn add(self, rhs: i32) -> Int3 {
        Int3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}
impl Add<Int3> for i32 {
    type Output = Int3;
    fn add(self, rhs: Int3) -> Int3 {
        Int3 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z }
    }
}
impl AddAssign<Int3> for Int3 {
    fn add_assign(&mut self, rhs: Int3) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z;
    }
}
impl AddAssign<i32> for Int3 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; self.y += rhs; self.z += rhs;
    }
}

impl Sub<Int3> for Int3 {
    type Output = Int3;
    fn sub(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Sub<i32> for Int3 {
    type Output = Int3;
    fn sub(self, rhs: i32) -> Int3 {
        Int3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}
impl Sub<Int3> for i32 {
    type Output = Int3;
    fn sub(self, rhs: Int3) -> Int3 {
        Int3 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z }
    }
}
impl SubAssign<Int3> for Int3 {
    fn sub_assign(&mut self, rhs: Int3) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z;
    }
}
impl SubAssign<i32> for Int3 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs;
    }
}

impl Mul<Int3> for Int3 {
    type Output = Int3;
    fn mul(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<i32> for Int3 {
    type Output = Int3;
    fn mul(self, rhs: i32) -> Int3 {
        Int3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Int3> for i32 {
    type Output = Int3;
    fn mul(self, rhs: Int3) -> Int3 {
        Int3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}
impl MulAssign<Int3> for Int3 {
    fn mul_assign(&mut self, rhs: Int3) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z;
    }
}
impl MulAssign<i32> for Int3 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}

impl Div<Int3> for Int3 {
    type Output = Int3;
    fn div(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<i32> for Int3 {
    type Output = Int3;
    fn div(self, rhs: i32) -> Int3 {
        Int3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl DivAssign<Int3> for Int3 {
    fn div_assign(&mut self, rhs: Int3) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z;
    }
}
impl DivAssign<i32> for Int3 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs;
    }
}

impl Rem<Int3> for Int3 {
    type Output = Int3;
    fn rem(self, rhs: Int3) -> Int3 {
        Int3 { x: self.x % rhs.x, y: self.y % rhs.y, z: self.z % rhs.z }
    }
}
impl Rem<i32> for Int3 {
    type Output = Int3;
    fn rem(self, rhs: i32) -> Int3 {
        Int3 { x: self.x % rhs, y: self.y % rhs, z: self.z % rhs }
    }
}
impl RemAssign<Int3> for Int3 {
    fn rem_assign(&mut self, rhs: Int3) {
        self.x %= rhs.x; self.y %= rhs.y; self.z %= rhs.z;
    }
}
impl RemAssign<i32> for Int3 {
    fn rem_assign(&mut self, rhs: i32) {
        self.x %= rhs; self.y %= rhs; self.z %= rhs;
    }
}

impl From<(i32, i32, i32)> for Int3 {
    fn from((x, y, z): (i32, i32, i32)) -> Int3 {
        Int3 { x, y, z }
    }
}
impl From<[i32; 3]> for Int3 {
    fn from([x, y, z]: [i32; 3]) -> Int3 {
        Int3 { x, y, z }
    }
}
impl From<Int3> for [i32; 3] {
    fn from(v: Int3) -> [i32; 3] {
        [v.x, v.y, v.z]
    }
}
