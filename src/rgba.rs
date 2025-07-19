//! RGBA color representation.
//!
//! Not a full featured implementation, but provides basic functionality for this crate.

#[cfg(feature = "std")]
extern crate std;

use crate::math;

/// Four-component vector type for representing RGBA colors.
///
/// ## Layout
///
/// The layout of the `Rgba` struct is as follows:
///
/// ```text
/// +----------------+----------------+----------------+----------------+
/// |      r         |      g         |      b         |      a         |
/// +----------------+----------------+----------------+----------------+
/// ```
///
/// As a C-style struct, it represents:
///
/// ```c
/// template <typename C>
/// struct Rgba {
///    C r;
///    C g;
///    C b;
///    C a;
/// };
/// ```
///
/// See [`U8x4Rgba`] and [`F32x4Rgba`] for type aliases with specific component types.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Rgba<C>
where
    C: Copy,
{
    /// Alpha component of the color.
    pub r: C,

    /// Blue component of the color.
    pub g: C,

    /// Green component of the color.
    pub b: C,

    /// Red component of the color.
    pub a: C,
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for U8x4Rgba {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for U8x4Rgba {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for F32x4Rgba {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for F32x4Rgba {}

impl<C> Rgba<C>
where
    C: Copy,
{
    /// Creates a new `Rgba` instance with the specified components.
    pub const fn new(r: C, g: C, b: C, a: C) -> Self {
        Self { r, g, b, a }
    }

    /// Returns the red component.
    pub const fn red(&self) -> C {
        self.r
    }

    /// Returns the green component.
    pub const fn green(&self) -> C {
        self.g
    }

    /// Returns the blue component.
    pub const fn blue(&self) -> C {
        self.b
    }

    /// Returns the alpha component.
    pub const fn alpha(&self) -> C {
        self.a
    }
}

/// Four-component RGBA color with a component type of [`u8`].
pub type U8x4Rgba = Rgba<u8>;

const MAX: f32 = 255.0;

impl U8x4Rgba {
    /// Creates a new `U8x4Rgba` instance with `0` for all components.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl F32x4Rgba {
    /// Creates a new `F32x4Rgba` instance with `0` for all components.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<U8x4Rgba> for F32x4Rgba {
    fn from(rgba: U8x4Rgba) -> Self {
        Self::new(
            f32::from(rgba.red()) / MAX,
            f32::from(rgba.green()) / MAX,
            f32::from(rgba.blue()) / MAX,
            f32::from(rgba.alpha()) / MAX,
        )
    }
}

/// Four-component RGBA color with a component type of [`f32`].
pub type F32x4Rgba = Rgba<f32>;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<F32x4Rgba> for U8x4Rgba {
    fn from(rgba: F32x4Rgba) -> Self {
        let r = math::round(rgba.red() * MAX);
        let g = math::round(rgba.green() * MAX);
        let b = math::round(rgba.blue() * MAX);
        let a = math::round(rgba.alpha() * MAX);
        Self::new(r as u8, g as u8, b as u8, a as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn u8_to_f32() {
        let rgba_u8 = U8x4Rgba::new(255, 128, 64, 32);
        let rgba_f32: F32x4Rgba = rgba_u8.into();
        assert_eq!(rgba_f32.red(), 1.0);
        assert_eq!(rgba_f32.green(), 0.501_960_8);
        assert_eq!(rgba_f32.blue(), 0.250_980_4);
        assert_eq!(rgba_f32.alpha(), 0.125_490_2);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_to_u8() {
        let rgba_f32 = F32x4Rgba::new(1.0, 0.501_960_8, 0.250_980_4, 0.0);
        let rgba_u8: U8x4Rgba = rgba_f32.into();
        assert_eq!(rgba_u8.red(), 255);
        assert_eq!(rgba_u8.green(), 128);
        assert_eq!(rgba_u8.blue(), 64);
        assert_eq!(rgba_u8.alpha(), 0);
    }
}
