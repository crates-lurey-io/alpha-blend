//! Alpha blending and compositing in (optionally) zero-dependency Rust.
//!
//! ## Features
//!
//! By default, this crate is `no_std` compatible, and uses [`libm`] for some math operations.
//!
//! ### `std`
//!
//! Uses the standard library for math operations, such as `f32::round`.
//!
//! Removes the dependency on `libm`.
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
