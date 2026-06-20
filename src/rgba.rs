//! RGBA color representation.

#[cfg(feature = "std")]
extern crate std;

use core::fmt;
use core::ptr;

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
    /// Red component.
    pub r: C,

    /// Green component.
    pub g: C,

    /// Blue component.
    pub b: C,

    /// Alpha component.
    pub a: C,
}

// ---------------------------------------------------------------------------
// `bytemuck` impls
// ---------------------------------------------------------------------------

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for U8x4Rgba {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for U8x4Rgba {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for F32x4Rgba {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for F32x4Rgba {}

// ---------------------------------------------------------------------------
// Structural conversions
// ---------------------------------------------------------------------------

impl<C: Copy> From<[C; 4]> for Rgba<C> {
    fn from([r, g, b, a]: [C; 4]) -> Self {
        Self::new(r, g, b, a)
    }
}

impl<C: Copy> From<Rgba<C>> for [C; 4] {
    fn from(c: Rgba<C>) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl<C: Copy> From<(C, C, C, C)> for Rgba<C> {
    fn from((r, g, b, a): (C, C, C, C)) -> Self {
        Self::new(r, g, b, a)
    }
}

impl<C: Copy> From<Rgba<C>> for (C, C, C, C) {
    fn from(c: Rgba<C>) -> Self {
        (c.r, c.g, c.b, c.a)
    }
}

impl<C: Copy> AsRef<[C]> for Rgba<C> {
    fn as_ref(&self) -> &[C] {
        // Safety: Rgba<C> is repr(C) with 4 contiguous elements of type C.
        unsafe { core::slice::from_raw_parts(ptr::from_ref(self).cast::<C>(), 4) }
    }
}

impl<C: Copy> AsMut<[C]> for Rgba<C> {
    fn as_mut(&mut self) -> &mut [C] {
        unsafe { core::slice::from_raw_parts_mut(ptr::from_mut(self).cast::<C>(), 4) }
    }
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for Rgba<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl fmt::Display for Rgba<f32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

// ---------------------------------------------------------------------------
// Eq + Hash for u8
// ---------------------------------------------------------------------------

impl Eq for Rgba<u8> {}

impl core::hash::Hash for Rgba<u8> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
        self.a.hash(state);
    }
}

// ---------------------------------------------------------------------------
// Rgba<C> inherent methods
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

/// Four-component RGBA color with a component type of [`u8`].
pub type U8x4Rgba = Rgba<u8>;

/// Four-component RGBA color with a component type of [`f32`].
pub type F32x4Rgba = Rgba<f32>;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

impl U8x4Rgba {
    /// Fully transparent black (`(0, 0, 0, 0)`).
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    /// Fully opaque black (`(0, 0, 0, 255)`).
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    /// Fully opaque white (`(255, 255, 255, 255)`).
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
}

impl F32x4Rgba {
    /// Fully transparent black (`(0.0, 0.0, 0.0, 0.0)`).
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Fully opaque black (`(0.0, 0.0, 0.0, 1.0)`).
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// Fully opaque white (`(1.0, 1.0, 1.0, 1.0)`).
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
}

// ---------------------------------------------------------------------------
// U8 helpers
// ---------------------------------------------------------------------------

const MAX: f32 = 255.0;

impl U8x4Rgba {
    /// Creates a new `U8x4Rgba` instance with `0` for all components.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Constructs an opaque `U8x4Rgba` from a `0x00RRGGBB` packed pixel.
    ///
    /// The alpha channel is set to 255 (fully opaque). The top byte of
    /// `pixel` is ignored.
    #[must_use]
    pub const fn from_rgb_u32(pixel: u32) -> Self {
        Self::new(
            ((pixel >> 16) & 0xFF) as u8,
            ((pixel >> 8) & 0xFF) as u8,
            (pixel & 0xFF) as u8,
            255,
        )
    }

    /// Packs this color into a `0x00RRGGBB` `u32`, discarding the alpha channel.
    #[must_use]
    pub const fn to_rgb_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Blends `self` (source) over `dst` (destination) using integer `SourceOver`.
    ///
    /// Equivalent to Porter-Duff `SRC_OVER`:
    /// `out = src * src.a + dst * (1 - src.a)`
    ///
    /// Uses the `(x + (x >> 8) + 1) >> 8` approximation for division by 255,
    /// which avoids floating-point and is exact for all inputs in range.
    #[must_use]
    pub fn source_over(self, dst: Self) -> Self {
        let a = u16::from(self.a);
        let inv_a = 255 - a;

        let blend_channel = |s: u8, d: u8| -> u8 {
            let v = u16::from(s) * a + u16::from(d) * inv_a;
            ((v + (v >> 8) + 1) >> 8) as u8
        };

        let out_a = {
            // Porter-Duff SRC_OVER: out_a = src_a + dst_a * (1 - src_a)
            // In integer form: (a * 255 + dst.a * (255 - a)) / 255
            let v = a * 255 + u16::from(dst.a) * inv_a;
            ((v + (v >> 8) + 1) >> 8) as u8
        };

        Self::new(
            blend_channel(self.r, dst.r),
            blend_channel(self.g, dst.g),
            blend_channel(self.b, dst.b),
            out_a,
        )
    }

    /// Returns `true` if this pixel is fully transparent (`alpha == 0`).
    #[must_use]
    pub const fn is_transparent(self) -> bool {
        self.a == 0
    }

    /// Returns `true` if this pixel is fully opaque (`alpha == 255`).
    #[must_use]
    pub const fn is_opaque(self) -> bool {
        self.a == 255
    }
}

// ---------------------------------------------------------------------------
// F32 helpers
// ---------------------------------------------------------------------------

impl F32x4Rgba {
    /// Creates a new `F32x4Rgba` instance with `0` for all components.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Clamps all channels to `[0.0, 1.0]`.
    ///
    /// Necessary after [`crate::BlendMode::Plus`], which can produce values > 1.0.
    #[must_use]
    pub const fn clamp(self) -> Self {
        // f32::clamp is not const in stable Rust, so we use manual min/max.
        #[inline]
        const fn clamp_f32(v: f32, lo: f32, hi: f32) -> f32 {
            if v < lo {
                lo
            } else if v > hi {
                hi
            } else {
                v
            }
        }
        Self::new(
            clamp_f32(self.r, 0.0, 1.0),
            clamp_f32(self.g, 0.0, 1.0),
            clamp_f32(self.b, 0.0, 1.0),
            clamp_f32(self.a, 0.0, 1.0),
        )
    }

    /// Converts from straight alpha to premultiplied alpha.
    ///
    /// `premultiplied.rgb = straight.rgb * straight.a`
    #[must_use]
    pub fn premultiply(self) -> Self {
        Self::new(self.r * self.a, self.g * self.a, self.b * self.a, self.a)
    }

    /// Converts from premultiplied alpha to straight alpha.
    ///
    /// No-op if `alpha == 0` (avoids division by zero).
    #[must_use]
    pub fn unpremultiply(self) -> Self {
        if self.a == 0.0 {
            return Self::TRANSPARENT;
        }
        Self::new(self.r / self.a, self.g / self.a, self.b / self.a, self.a)
    }

    /// Linearly interpolates between `self` and `other` by `t` (clamped to `[0.0, 1.0]`).
    ///
    /// `t = 0.0` returns `self`; `t = 1.0` returns `other`.
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }
}

// ---------------------------------------------------------------------------
// u8 ↔ f32 conversion
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::cast_lossless, clippy::float_cmp)]
mod tests {
    use super::*;

    // --- Bug fix: field doc comments ---

    #[test]
    fn field_order_is_rgba() {
        let c = U8x4Rgba::new(1, 2, 3, 4);
        assert_eq!(c.r, 1);
        assert_eq!(c.g, 2);
        assert_eq!(c.b, 3);
        assert_eq!(c.a, 4);
    }

    // --- u8 ↔ f32 round-trip ---

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

    // --- Named constants ---

    #[test]
    fn u8_named_constants() {
        assert_eq!(U8x4Rgba::TRANSPARENT, U8x4Rgba::new(0, 0, 0, 0));
        assert_eq!(U8x4Rgba::BLACK, U8x4Rgba::new(0, 0, 0, 255));
        assert_eq!(U8x4Rgba::WHITE, U8x4Rgba::new(255, 255, 255, 255));
    }

    #[test]
    fn f32_named_constants() {
        assert_eq!(F32x4Rgba::TRANSPARENT, F32x4Rgba::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(F32x4Rgba::BLACK, F32x4Rgba::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(F32x4Rgba::WHITE, F32x4Rgba::new(1.0, 1.0, 1.0, 1.0));
    }

    // --- u32 packed pixel helpers ---

    #[test]
    fn from_rgb_u32_round_trips() {
        let pixel = 0x00FF_8040u32;
        let rgba = U8x4Rgba::from_rgb_u32(pixel);
        assert_eq!(rgba.r, 0xFF);
        assert_eq!(rgba.g, 0x80);
        assert_eq!(rgba.b, 0x40);
        assert_eq!(rgba.a, 255);
        assert_eq!(rgba.to_rgb_u32(), pixel);
    }

    #[test]
    fn from_rgb_u32_ignores_top_byte() {
        assert_eq!(
            U8x4Rgba::from_rgb_u32(0xDEAD_BEEF).to_rgb_u32(),
            U8x4Rgba::from_rgb_u32(0x00AD_BEEF).to_rgb_u32(),
        );
    }

    #[test]
    fn to_rgb_u32_discards_alpha() {
        let rgba = U8x4Rgba::new(0x10, 0x20, 0x30, 0xAB);
        assert_eq!(rgba.to_rgb_u32(), 0x0010_2030);
    }

    // --- integer source_over ---

    #[test]
    fn source_over_opaque_src_returns_src() {
        let src = U8x4Rgba::new(255, 0, 0, 255);
        let dst = U8x4Rgba::new(0, 0, 255, 255);
        let out = src.source_over(dst);
        assert_eq!(out, src);
    }

    #[test]
    fn source_over_transparent_src_returns_dst() {
        let src = U8x4Rgba::new(255, 0, 0, 0);
        let dst = U8x4Rgba::new(0, 0, 255, 255);
        let out = src.source_over(dst);
        assert_eq!(out, dst);
    }

    #[test]
    fn source_over_half_alpha_blends() {
        let src = U8x4Rgba::new(0, 255, 0, 128);
        let dst = U8x4Rgba::new(255, 0, 0, 255);
        let out = src.source_over(dst);
        assert!((out.r as i16 - 127).abs() <= 1, "r={}", out.r);
        assert!((out.g as i16 - 128).abs() <= 1, "g={}", out.g);
        assert_eq!(out.b, 0);
        // Standard SRC_OVER: out_a = src_a + dst_a * (1 - src_a) = 0.5 + 1.0 * 0.5 = 1.0 -> 255
        assert_eq!(out.a, 255);
    }

    #[test]
    fn source_over_expected_values() {
        // Standard SRC_OVER formula:
        //   out_a = src_a + dst_a * (1 - src_a)
        //   out_rgb = (src_rgb * src_a + dst_rgb * (1 - src_a)) / 255
        // where / uses the (x + (x>>8) + 1) >> 8 integer approximation.
        let cases: &[(U8x4Rgba, U8x4Rgba, U8x4Rgba)] = &[
            // Opaque src over opaque dst -> src wins
            (
                U8x4Rgba::new(200, 100, 50, 255),
                U8x4Rgba::new(10, 20, 30, 255),
                U8x4Rgba::new(200, 100, 50, 255),
            ),
            // Fully transparent -> dst wins entirely
            (U8x4Rgba::TRANSPARENT, U8x4Rgba::WHITE, U8x4Rgba::WHITE),
            // 50% green over opaque red
            (
                U8x4Rgba::new(0, 255, 0, 128),
                U8x4Rgba::new(255, 0, 0, 255),
                U8x4Rgba::new(127, 128, 0, 255),
            ),
            // Opaque white over transparent -> src wins
            (U8x4Rgba::WHITE, U8x4Rgba::TRANSPARENT, U8x4Rgba::WHITE),
        ];
        for &(src, dst, expected) in cases {
            let out = src.source_over(dst);
            assert!(
                (out.r as i16 - expected.r as i16).abs() <= 2
                    && (out.g as i16 - expected.g as i16).abs() <= 2
                    && (out.b as i16 - expected.b as i16).abs() <= 2
                    && (out.a as i16 - expected.a as i16).abs() <= 2,
                "src={src} dst={dst}: got {out}, expected {expected}",
            );
        }
    }

    #[test]
    fn source_over_both_transparent() {
        let out = U8x4Rgba::TRANSPARENT.source_over(U8x4Rgba::TRANSPARENT);
        assert_eq!(out, U8x4Rgba::TRANSPARENT);
    }

    #[test]
    fn is_transparent_opaque() {
        assert!(U8x4Rgba::TRANSPARENT.is_transparent());
        assert!(!U8x4Rgba::BLACK.is_transparent());
        assert!(!U8x4Rgba::new(0, 0, 0, 1).is_transparent());
        assert!(U8x4Rgba::new(255, 0, 0, 0).is_transparent());
    }

    #[test]
    fn is_opaque() {
        assert!(U8x4Rgba::BLACK.is_opaque());
        assert!(U8x4Rgba::WHITE.is_opaque());
        assert!(!U8x4Rgba::TRANSPARENT.is_opaque());
        assert!(!U8x4Rgba::new(0, 0, 0, 254).is_opaque());
    }

    // --- From array / tuple / AsRef / AsMut ---

    #[test]
    fn from_array() {
        let rgba = Rgba::from([1u8, 2, 3, 4]);
        assert_eq!(rgba, U8x4Rgba::new(1, 2, 3, 4));
    }

    #[test]
    fn into_array() {
        let arr: [u8; 4] = U8x4Rgba::new(1, 2, 3, 4).into();
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn from_tuple() {
        let rgba = Rgba::from((1u8, 2, 3, 4));
        assert_eq!(rgba, U8x4Rgba::new(1, 2, 3, 4));
    }

    #[test]
    fn into_tuple() {
        let tup: (u8, u8, u8, u8) = U8x4Rgba::new(1, 2, 3, 4).into();
        assert_eq!(tup, (1, 2, 3, 4));
    }

    #[test]
    fn as_ref_slice() {
        let rgba = U8x4Rgba::new(1, 2, 3, 4);
        assert_eq!(rgba.as_ref(), &[1u8, 2, 3, 4]);
    }

    #[test]
    fn as_mut_slice() {
        let mut rgba = U8x4Rgba::new(1, 2, 3, 4);
        rgba.as_mut()[0] = 10;
        assert_eq!(rgba.r, 10);
    }

    // --- Display ---

    #[test]
    fn display_u8() {
        let s = format!("{}", U8x4Rgba::new(10, 20, 30, 40));
        assert_eq!(s, "rgba(10, 20, 30, 40)");
    }

    #[test]
    fn display_f32() {
        let s = format!("{}", F32x4Rgba::new(0.1, 0.2, 0.3, 1.0));
        assert_eq!(s, "rgba(0.1, 0.2, 0.3, 1)");
    }

    // --- Eq + Hash ---

    #[test]
    fn eq_hash_set() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(U8x4Rgba::new(1, 2, 3, 4));
        set.insert(U8x4Rgba::new(1, 2, 3, 4));
        assert_eq!(set.len(), 1);
        set.insert(U8x4Rgba::new(5, 6, 7, 8));
        assert_eq!(set.len(), 2);
    }

    // --- F32 helpers ---

    #[test]
    fn clamp_values() {
        let c = F32x4Rgba::new(1.5, -0.5, 2.0, -1.0).clamp();
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 1.0);
        assert_eq!(c.a, 0.0);
    }

    #[test]
    fn clamp_noop_for_valid() {
        let c = F32x4Rgba::new(0.5, 0.5, 0.5, 1.0).clamp();
        assert_eq!(c, F32x4Rgba::new(0.5, 0.5, 0.5, 1.0));
    }

    #[test]
    fn premultiply_identity_when_opaque() {
        let c = F32x4Rgba::new(0.5, 0.5, 0.5, 1.0);
        assert_eq!(c.premultiply(), c);
    }

    #[test]
    fn premultiply_half_alpha() {
        let c = F32x4Rgba::new(0.5, 1.0, 0.0, 0.5).premultiply();
        assert_eq!(c.r, 0.25);
        assert_eq!(c.g, 0.5);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 0.5);
    }

    #[test]
    fn unpremultiply_round_trips() {
        let orig = F32x4Rgba::new(0.5, 0.5, 0.5, 0.5);
        let pm = orig.premultiply();
        let back = pm.unpremultiply();
        assert!((back.r - orig.r).abs() < 1e-6);
        assert!((back.g - orig.g).abs() < 1e-6);
        assert!((back.b - orig.b).abs() < 1e-6);
        assert!((back.a - orig.a).abs() < 1e-6);
    }

    #[test]
    fn unpremultiply_transparent_returns_transparent() {
        let c = F32x4Rgba::new(0.5, 0.5, 0.5, 0.0).unpremultiply();
        assert_eq!(c, F32x4Rgba::TRANSPARENT);
    }

    #[test]
    fn lerp_identity() {
        let a = F32x4Rgba::new(0.2, 0.4, 0.6, 0.8);
        assert_eq!(a.lerp(F32x4Rgba::WHITE, 0.0), a);
    }

    #[test]
    fn lerp_full() {
        let a = F32x4Rgba::new(0.2, 0.4, 0.6, 0.8);
        assert_eq!(a.lerp(F32x4Rgba::WHITE, 1.0), F32x4Rgba::WHITE);
    }

    #[test]
    fn lerp_midpoint() {
        let a = F32x4Rgba::new(0.0, 0.0, 0.0, 1.0);
        let b = F32x4Rgba::new(1.0, 1.0, 1.0, 1.0);
        let mid = a.lerp(b, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-6);
        assert!((mid.g - 0.5).abs() < 1e-6);
        assert!((mid.b - 0.5).abs() < 1e-6);
        assert!((mid.a - 1.0).abs() < 1e-6);
    }
}
