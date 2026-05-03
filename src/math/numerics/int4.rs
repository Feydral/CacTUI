use std::ops::*;

use crate::math::numerics::{Float4, Int2, Int3, UInt4};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Int4 {
    pub const ZERO: Int4 = Int4::new(0, 0, 0, 0);
    pub const ONE: Int4 = Int4::new(1, 1, 1, 1);
    pub const UNIT_X: Int4 = Int4::new(1, 0, 0, 0);
    pub const UNIT_Y: Int4 = Int4::new(0, 1, 0, 0);
    pub const UNIT_Z: Int4 = Int4::new(0, 0, 1, 0);
    pub const UNIT_W: Int4 = Int4::new(0, 0, 0, 1);

    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Int4 {
        Int4 { x, y, z, w }
    }

    pub const fn splat(v: i32) -> Int4 {
        Int4 { x: v, y: v, z: v, w: v }
    }

    pub fn from2(xy: Int2, z: i32, w: i32) -> Int4 {
        Int4 { x: xy.x, y: xy.y, z, w }
    }

    pub fn from3(xyz: Int3, w: i32) -> Int4 {
        Int4 { x: xyz.x, y: xyz.y, z: xyz.z, w }
    }

    pub fn xx(self) -> Int2 { Int2::new(self.x, self.x) }
    pub fn xy(self) -> Int2 { Int2::new(self.x, self.y) }
    pub fn xz(self) -> Int2 { Int2::new(self.x, self.z) }
    pub fn xw(self) -> Int2 { Int2::new(self.x, self.w) }
    pub fn yx(self) -> Int2 { Int2::new(self.y, self.x) }
    pub fn yy(self) -> Int2 { Int2::new(self.y, self.y) }
    pub fn yz(self) -> Int2 { Int2::new(self.y, self.z) }
    pub fn yw(self) -> Int2 { Int2::new(self.y, self.w) }
    pub fn zx(self) -> Int2 { Int2::new(self.z, self.x) }
    pub fn zy(self) -> Int2 { Int2::new(self.z, self.y) }
    pub fn zz(self) -> Int2 { Int2::new(self.z, self.z) }
    pub fn zw(self) -> Int2 { Int2::new(self.z, self.w) }
    pub fn wx(self) -> Int2 { Int2::new(self.w, self.x) }
    pub fn wy(self) -> Int2 { Int2::new(self.w, self.y) }
    pub fn wz(self) -> Int2 { Int2::new(self.w, self.z) }
    pub fn ww(self) -> Int2 { Int2::new(self.w, self.w) }

    pub fn xyz(self) -> Int3 { Int3::new(self.x, self.y, self.z) }
    pub fn xyw(self) -> Int3 { Int3::new(self.x, self.y, self.w) }
    pub fn xzw(self) -> Int3 { Int3::new(self.x, self.z, self.w) }
    pub fn yzw(self) -> Int3 { Int3::new(self.y, self.z, self.w) }

    #[inline(always)]
    pub fn min(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x.min(rhs.x), y: self.y.min(rhs.y), z: self.z.min(rhs.z), w: self.w.min(rhs.w) }
    }

    #[inline(always)]
    pub fn max(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x.max(rhs.x), y: self.y.max(rhs.y), z: self.z.max(rhs.z), w: self.w.max(rhs.w) }
    }

    #[inline(always)]
    pub fn clamp(self, min: Int4, max: Int4) -> Int4 {
        self.max(min).min(max)
    }

    #[inline(always)]
    pub fn abs(self) -> Int4 {
        Int4 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs(), w: self.w.abs() }
    }

    #[inline(always)]
    pub fn signum(self) -> Int4 {
        Int4 { x: self.x.signum(), y: self.y.signum(), z: self.z.signum(), w: self.w.signum() }
    }

    #[inline(always)]
    pub fn dot(self, rhs: Int4) -> i32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline(always)]
    pub fn to_uint4(self) -> UInt4 {
        UInt4 { x: self.x as u32, y: self.y as u32, z: self.z as u32, w: self.w as u32 }
    }

    #[inline(always)]
    pub fn to_float4(self) -> Float4 {
        Float4 { x: self.x as f32, y: self.y as f32, z: self.z as f32, w: self.w as f32 }
    }
}

impl Neg for Int4 {
    type Output = Int4;
    fn neg(self) -> Int4 {
        Int4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl Add<Int4> for Int4 {
    type Output = Int4;
    fn add(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}
impl Add<i32> for Int4 {
    type Output = Int4;
    fn add(self, rhs: i32) -> Int4 {
        Int4 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs }
    }
}
impl Add<Int4> for i32 {
    type Output = Int4;
    fn add(self, rhs: Int4) -> Int4 {
        Int4 { x: self + rhs.x, y: self + rhs.y, z: self + rhs.z, w: self + rhs.w }
    }
}
impl AddAssign<Int4> for Int4 {
    fn add_assign(&mut self, rhs: Int4) {
        self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; self.w += rhs.w;
    }
}
impl AddAssign<i32> for Int4 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; self.y += rhs; self.z += rhs; self.w += rhs;
    }
}

impl Sub<Int4> for Int4 {
    type Output = Int4;
    fn sub(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}
impl Sub<i32> for Int4 {
    type Output = Int4;
    fn sub(self, rhs: i32) -> Int4 {
        Int4 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}
impl Sub<Int4> for i32 {
    type Output = Int4;
    fn sub(self, rhs: Int4) -> Int4 {
        Int4 { x: self - rhs.x, y: self - rhs.y, z: self - rhs.z, w: self - rhs.w }
    }
}
impl SubAssign<Int4> for Int4 {
    fn sub_assign(&mut self, rhs: Int4) {
        self.x -= rhs.x; self.y -= rhs.y; self.z -= rhs.z; self.w -= rhs.w;
    }
}
impl SubAssign<i32> for Int4 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; self.y -= rhs; self.z -= rhs; self.w -= rhs;
    }
}

impl Mul<Int4> for Int4 {
    type Output = Int4;
    fn mul(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}
impl Mul<i32> for Int4 {
    type Output = Int4;
    fn mul(self, rhs: i32) -> Int4 {
        Int4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}
impl Mul<Int4> for i32 {
    type Output = Int4;
    fn mul(self, rhs: Int4) -> Int4 {
        Int4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
    }
}
impl MulAssign<Int4> for Int4 {
    fn mul_assign(&mut self, rhs: Int4) {
        self.x *= rhs.x; self.y *= rhs.y; self.z *= rhs.z; self.w *= rhs.w;
    }
}
impl MulAssign<i32> for Int4 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs; self.w *= rhs;
    }
}

impl Div<Int4> for Int4 {
    type Output = Int4;
    fn div(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}
impl Div<i32> for Int4 {
    type Output = Int4;
    fn div(self, rhs: i32) -> Int4 {
        Int4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}
impl DivAssign<Int4> for Int4 {
    fn div_assign(&mut self, rhs: Int4) {
        self.x /= rhs.x; self.y /= rhs.y; self.z /= rhs.z; self.w /= rhs.w;
    }
}
impl DivAssign<i32> for Int4 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; self.y /= rhs; self.z /= rhs; self.w /= rhs;
    }
}

impl Rem<Int4> for Int4 {
    type Output = Int4;
    fn rem(self, rhs: Int4) -> Int4 {
        Int4 { x: self.x % rhs.x, y: self.y % rhs.y, z: self.z % rhs.z, w: self.w % rhs.w }
    }
}
impl Rem<i32> for Int4 {
    type Output = Int4;
    fn rem(self, rhs: i32) -> Int4 {
        Int4 { x: self.x % rhs, y: self.y % rhs, z: self.z % rhs, w: self.w % rhs }
    }
}
impl RemAssign<Int4> for Int4 {
    fn rem_assign(&mut self, rhs: Int4) {
        self.x %= rhs.x; self.y %= rhs.y; self.z %= rhs.z; self.w %= rhs.w;
    }
}
impl RemAssign<i32> for Int4 {
    fn rem_assign(&mut self, rhs: i32) {
        self.x %= rhs; self.y %= rhs; self.z %= rhs; self.w %= rhs;
    }
}

impl From<(i32, i32, i32, i32)> for Int4 {
    fn from((x, y, z, w): (i32, i32, i32, i32)) -> Int4 {
        Int4 { x, y, z, w }
    }
}
impl From<[i32; 4]> for Int4 {
    fn from([x, y, z, w]: [i32; 4]) -> Int4 {
        Int4 { x, y, z, w }
    }
}
impl From<Int4> for [i32; 4] {
    fn from(v: Int4) -> [i32; 4] {
        [v.x, v.y, v.z, v.w]
    }
}
