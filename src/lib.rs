//! Alpha blending and compositing in (optionally) zero-dependency Rust.
//!
//! All blending operations assume **straight (un-premultiplied) alpha** unless
//! documented otherwise.  For premultiplied conversions see
//! [`crate::rgba::F32x4Rgba::premultiply`] and [`crate::rgba::F32x4Rgba::unpremultiply`].
//!
//! ## Examples
//!
//! The [`RgbaBlend`] trait defines a method for blending two RGBA colors together, and the
//! [`BlendMode`] enum provides various blending modes based on Porter-Duff coefficients; for
//! example [`BlendMode::SourceOver`] blends the source color over the destination color using
//! [`PorterDuff::SRC_OVER`][].
//!
//! [`PorterDuff::SRC_OVER`]: `crate::porter_duff::PorterDuff::SRC_OVER`
//!
//! ```rust
//! use alpha_blend::{rgba::F32x4Rgba, BlendMode, RgbaBlend};
//!
//! let src = F32x4Rgba { r: 1.0, g: 0.0, b: 0.0, a: 0.5 }; // Semi-transparent red
//! let dst = F32x4Rgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }; // Opaque blue
//! let blended = BlendMode::SourceOver.apply(src, dst);
//!
//! // A mixed color of red and blue with alpha blending
//! assert_eq!(blended, F32x4Rgba { r: 0.5, g: 0.0, b: 0.5, a: 0.75 });
//! ```
//!
//! ## Features
//!
//! By default, this crate is `no_std` compatible, and uses [`libm`] for some math operations.
//!
//! Either `std` or `libm` must be enabled.
//!
//! ### `bytemuck`
//!
//! Enables the `bytemuck` crate for zero-copy conversions between types.
//!
//! ### `libm`
//!
//! _This feature is enabled by default._
//!
//! Uses the `libm` crate for math operations.
//!
//! ### `libm-arch`
//!
//! _This feature is enabled by default._
//!
//! Enables the `arch` feature of `libm`.
//!
//! ### `std`
//!
//! Uses the standard library for math operations, such as `f32::round`.

#![cfg_attr(not(feature = "std"), no_std)]

use crate::{porter_duff::PorterDuff, rgba::Rgba};

pub(crate) mod math;
pub mod porter_duff;
pub mod rgba;
pub(crate) mod vec4;

/// Supported blend modes by this crate.
///
/// All modes operate in **straight alpha**.  See
/// [`F32x4Rgba::premultiply`](crate::rgba::F32x4Rgba::premultiply) for premultiplied support.
///
/// ## Overflow
///
/// [`Plus`](BlendMode::Plus) can produce channel values > 1.0.  Call
/// [`clamp()`](crate::rgba::F32x4Rgba::clamp) on the result when using `Plus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlendMode {
    /// Destination pixels covered by the source pixels are cleared.
    Clear,

    /// Source pixels are copied to the destination.
    Source,

    /// Destination pixels are copied to the source.
    Destination,

    /// Source pixels are copied to the destination, ignoring the alpha channel.
    #[default]
    SourceOver,

    /// Destination pixels are copied to the source, ignoring the alpha channel.
    DestinationOver,

    /// Source pixels are copied to the destination, only where the destination is opaque.
    SourceIn,

    /// Destination pixels are copied to the source, only where the source is opaque.
    DestinationIn,

    /// Source pixels are copied to the destination, only where the source is opaque.
    SourceOut,

    /// Destination pixels are copied to the source, only where the destination is opaque.
    DestinationOut,

    /// Source pixels are copied to the destination, where both source and destination are opaque.
    SourceAtop,

    /// Destination pixels are copied to the source, where both source and destination are opaque.
    DestinationAtop,

    /// Source pixels are blended with the destination using the source's alpha channel.
    Xor,

    /// Source pixels are added to the destination.
    ///
    /// **Note**: can produce channel values > 1.0.  Call
    /// [`clamp()`](crate::rgba::F32x4Rgba::clamp) on the result when clamping is needed.
    Plus,
}

impl RgbaBlend for BlendMode {
    type Channel = f32;

    fn apply(&self, src: Rgba<Self::Channel>, dst: Rgba<Self::Channel>) -> Rgba<Self::Channel> {
        let pd: PorterDuff<f32, fn(f32, f32) -> f32> = match self {
            Self::Clear => PorterDuff::CLEAR,
            Self::Source => PorterDuff::SRC,
            Self::Destination => PorterDuff::DST,
            Self::SourceOver => PorterDuff::SRC_OVER,
            Self::DestinationOver => PorterDuff::DST_OVER,
            Self::SourceIn => PorterDuff::SRC_IN,
            Self::DestinationIn => PorterDuff::DST_IN,
            Self::SourceOut => PorterDuff::SRC_OUT,
            Self::DestinationOut => PorterDuff::DST_OUT,
            Self::SourceAtop => PorterDuff::SRC_ATOP,
            Self::DestinationAtop => PorterDuff::DST_ATOP,
            Self::Xor => PorterDuff::XOR,
            Self::Plus => PorterDuff::PLUS,
        };
        pd.apply(src, dst)
    }
}

/// Blends pixel colors using alpha compositing.
pub trait RgbaBlend {
    /// What type of channel this blend mode operates on.
    ///
    /// **Note**: only `f32` is currently supported via the provided
    /// [`BlendMode`] / [`PorterDuff`] implementations.  `u8` blending is
    /// available directly on [`U8x4Rgba`](crate::rgba::U8x4Rgba) via
    /// [`source_over`](crate::rgba::U8x4Rgba::source_over).
    type Channel: Copy;

    /// Blends two colors together using this blend mode.
    fn apply(&self, src: Rgba<Self::Channel>, dst: Rgba<Self::Channel>) -> Rgba<Self::Channel>;

    /// Blend `src` over `dst` in place, pixel by pixel.
    ///
    /// Default impl calls [`apply`](RgbaBlend::apply) in a loop.
    /// Implementations may override with SIMD or other optimized paths.
    fn apply_slice(&self, src: &[Rgba<Self::Channel>], dst: &mut [Rgba<Self::Channel>]) {
        assert_eq!(
            src.len(),
            dst.len(),
            "src and dst slices must have the same length"
        );
        for (s, d) in src.iter().zip(dst.iter_mut()) {
            *d = self.apply(*s, *d);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgba::F32x4Rgba;

    #[test]
    fn blend_mode_default_is_source_over() {
        assert_eq!(BlendMode::default(), BlendMode::SourceOver);
    }

    #[test]
    fn apply_slice_matches_individual() {
        let src = [
            F32x4Rgba::new(1.0, 0.0, 0.0, 0.5),
            F32x4Rgba::new(0.0, 1.0, 0.0, 1.0),
        ];
        let dst = [
            F32x4Rgba::new(0.0, 0.0, 1.0, 1.0),
            F32x4Rgba::new(1.0, 0.0, 0.0, 1.0),
        ];

        let mut batch = dst;
        BlendMode::SourceOver.apply_slice(&src, &mut batch);

        for (i, (s, d)) in src.iter().zip(dst.iter()).enumerate() {
            let expected = BlendMode::SourceOver.apply(*s, *d);
            assert_eq!(batch[i], expected);
        }
    }

    #[test]
    #[should_panic(expected = "must have the same length")]
    fn apply_slice_panics_on_mismatched_lengths() {
        let src = [F32x4Rgba::new(0.0, 0.0, 0.0, 0.0)];
        let mut dst = [F32x4Rgba::new(1.0, 1.0, 1.0, 1.0); 2];
        BlendMode::SourceOver.apply_slice(&src, &mut dst);
    }

    #[test]
    fn blend_mode_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BlendMode::SourceOver);
        set.insert(BlendMode::SourceOver);
        assert_eq!(set.len(), 1);
        set.insert(BlendMode::Clear);
        assert_eq!(set.len(), 2);
    }
}
