//! Alpha blending and compositing in (optionally) zero-dependency Rust.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Destination pixels covered by the source pixels are cleared.
    Clear,

    /// Source pixels are copied to the destination.
    Source,

    /// Destination pixels are copied to the source.
    Destination,

    /// Source pixels are copied to the destination, ignoring the alpha channel.
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
    Plus,
}

impl BlendMode {
    /// Returns an [`RgbaBlend`] implementation for this blend mode.
    #[must_use]
    fn as_rgba_blend_f32(&self) -> impl RgbaBlend<Channel = f32> {
        match self {
            BlendMode::Clear => PorterDuff::CLEAR,
            BlendMode::Source => PorterDuff::SRC,
            BlendMode::Destination => PorterDuff::DST,
            BlendMode::SourceOver => PorterDuff::SRC_OVER,
            BlendMode::DestinationOver => PorterDuff::DST_OVER,
            BlendMode::SourceIn => PorterDuff::SRC_IN,
            BlendMode::DestinationIn => PorterDuff::DST_IN,
            BlendMode::SourceOut => PorterDuff::SRC_OUT,
            BlendMode::DestinationOut => PorterDuff::DST_OUT,
            BlendMode::SourceAtop => PorterDuff::SRC_ATOP,
            BlendMode::DestinationAtop => PorterDuff::DST_ATOP,
            BlendMode::Xor => PorterDuff::XOR,
            BlendMode::Plus => PorterDuff::PLUS,
        }
    }
}

impl RgbaBlend for BlendMode {
    type Channel = f32;

    fn apply(&self, src: Rgba<Self::Channel>, dst: Rgba<Self::Channel>) -> Rgba<Self::Channel> {
        self.as_rgba_blend_f32().apply(src, dst)
    }
}

/// Blends pixel colors using alpha compositing.
pub trait RgbaBlend {
    /// What type of channel this blend mode operates on.
    type Channel: Copy;

    /// Blends two colors together using this blend mode.
    fn apply(&self, src: Rgba<Self::Channel>, dst: Rgba<Self::Channel>) -> Rgba<Self::Channel>;
}
