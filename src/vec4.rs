use core::{
    mem,
    ops::{Add, Mul},
};

use crate::rgba::F32x4Rgba;

/// Vector with four [`f32`] components.
pub struct F32x4 {
    /// The `w` lane, the first component.
    pub w: f32,

    /// The `x` lane, the second component.
    pub x: f32,

    /// The `y` lane, the third component.
    pub y: f32,

    /// The `z` lane, the fourth component.
    pub z: f32,
}

impl From<F32x4Rgba> for F32x4 {
    fn from(rgba: F32x4Rgba) -> Self {
        unsafe { mem::transmute(rgba) }
    }
}

impl From<F32x4> for F32x4Rgba {
    fn from(vec: F32x4) -> Self {
        unsafe { mem::transmute(vec) }
    }
}

impl F32x4 {
    /// Creates a new `F32x4` instance with the specified components.
    pub const fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    /// Creates a new `Cx4` instance with all components set to zero (`0.0`)
    #[must_use]
    pub const fn zeroed() -> Self {
        Self {
            w: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a new `Cx4` instance with all components set to the given value.
    #[must_use]
    pub const fn splat(value: f32) -> Self {
        Self {
            w: value,
            x: value,
            y: value,
            z: value,
        }
    }

    /// Returns the RGBA-equivalent of this `Cx4<f32>`.
    #[must_use]
    pub const fn into_rgba(self) -> F32x4Rgba {
        unsafe { mem::transmute(self) }
    }
}

impl Add<f32> for F32x4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        F32x4 {
            w: self.w + rhs,
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<F32x4> for F32x4 {
    type Output = Self;

    fn add(self, rhs: F32x4) -> Self::Output {
        F32x4 {
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f32> for F32x4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        F32x4 {
            w: self.w * rhs,
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<F32x4> for F32x4 {
    type Output = Self;

    fn mul(self, rhs: F32x4) -> Self::Output {
        F32x4 {
            w: self.w * rhs.w,
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_f32x4_rgba_to_f32x4() {
        let rgba = F32x4Rgba::new(0.1, 0.2, 0.3, 0.4);
        let vec: F32x4 = rgba.into();
        assert_eq!(vec.w, 0.1);
        assert_eq!(vec.x, 0.2);
        assert_eq!(vec.y, 0.3);
        assert_eq!(vec.z, 0.4);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn from_f32x4_to_f32x4_rgba() {
        let vec = F32x4::new(0.1, 0.2, 0.3, 0.4);
        let rgba: F32x4Rgba = vec.into();
        assert_eq!(rgba.red(), 0.1);
        assert_eq!(rgba.green(), 0.2);
        assert_eq!(rgba.blue(), 0.3);
        assert_eq!(rgba.alpha(), 0.4);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_new() {
        let vec = F32x4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vec.w, 1.0);
        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, 3.0);
        assert_eq!(vec.z, 4.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_zeroed() {
        let vec = F32x4::zeroed();
        assert_eq!(vec.w, 0.0);
        assert_eq!(vec.x, 0.0);
        assert_eq!(vec.y, 0.0);
        assert_eq!(vec.z, 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_splat() {
        let vec = F32x4::splat(5.0);
        assert_eq!(vec.w, 5.0);
        assert_eq!(vec.x, 5.0);
        assert_eq!(vec.y, 5.0);
        assert_eq!(vec.z, 5.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_into_rgba() {
        let vec = F32x4::new(0.1, 0.2, 0.3, 0.4);
        let rgba = vec.into_rgba();
        assert_eq!(rgba.red(), 0.1);
        assert_eq!(rgba.green(), 0.2);
        assert_eq!(rgba.blue(), 0.3);
        assert_eq!(rgba.alpha(), 0.4);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_add_f32() {
        let vec = F32x4::new(1.0, 2.0, 3.0, 4.0);
        let result = vec + 1.5;
        assert_eq!(result.w, 2.5);
        assert_eq!(result.x, 3.5);
        assert_eq!(result.y, 4.5);
        assert_eq!(result.z, 5.5);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_add_f32x4() {
        let vec1 = F32x4::new(1.0, 2.0, 3.0, 4.0);
        let vec2 = F32x4::new(5.0, 6.0, 7.0, 8.0);
        let result = vec1 + vec2;
        assert_eq!(result.w, 6.0);
        assert_eq!(result.x, 8.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, 12.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_mul_f32() {
        let vec = F32x4::new(1.0, 2.0, 3.0, 4.0);
        let result = vec * 2.0;
        assert_eq!(result.w, 2.0);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 8.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32x4_mul_f32x4() {
        let vec1 = F32x4::new(1.0, 2.0, 3.0, 4.0);
        let vec2 = F32x4::new(5.0, 6.0, 7.0, 8.0);
        let result = vec1 * vec2;
        assert_eq!(result.w, 5.0);
        assert_eq!(result.x, 12.0);
        assert_eq!(result.y, 21.0);
        assert_eq!(result.z, 32.0);
    }
}
