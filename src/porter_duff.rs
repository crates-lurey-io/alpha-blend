//! Alpha blending using Porter-Duff coefficients.
//!
//! This module provides a [`BlendMode`] implementation that uses Porter-Duff coefficients.
//!
//! [`BlendMode`]: crate::BlendMode

use core::marker::PhantomData;

use crate::{
    RgbaBlend,
    rgba::{F32x4Rgba, Rgba},
    vec4::F32x4,
};

/// A [`BlendMode`][] that uses [Porter-Duff coefficients] to blend colors.
///
/// [`BlendMode`]: crate::BlendMode
/// [Porter-Duff coefficients]: https://en.wikipedia.org/wiki/Alpha_compositing#Alpha_blending
pub struct PorterDuff<C, F: Fn(C, C) -> C> {
    /// Computes the source alpha given the source and destination alpha values.
    src: F,

    /// Computes the destination alpha given the source and destination alpha values.
    dst: F,

    /// Type of the alpha values used in blending.
    _ty: PhantomData<C>,
}

impl<C, F: Fn(C, C) -> C> PorterDuff<C, F> {
    const fn new(src: F, dst: F) -> Self {
        Self {
            src,
            dst,
            _ty: PhantomData,
        }
    }
}

impl PorterDuff<f32, fn(f32, f32) -> f32> {
    /// Returns the result of the blend operation using source and destination alpha values.
    #[must_use]
    pub fn blend(&self, src: F32x4Rgba, dst: F32x4Rgba) -> F32x4Rgba {
        let src_a = F32x4::splat((self.src)(src.alpha(), dst.alpha()));
        let dst_a = F32x4::splat((self.dst)(src.alpha(), dst.alpha()));
        let blend: F32x4 = src_a * F32x4::from(src) + dst_a * F32x4::from(dst);
        blend.into_rgba()
    }

    /// Always returns zero (`0.0`) regardless of the source and destination alpha values.
    const FN_ZERO: fn(f32, f32) -> f32 = |_, _| 0.0;

    /// Always returns one (`1.0`) regardless of the source and destination alpha values.
    const FN_ONE: fn(f32, f32) -> f32 = |_, _| 1.0;

    /// Returns the source alpha value, ignoring the destination alpha value.
    const FN_SRC: fn(f32, f32) -> f32 = |src, _| src;

    /// Returns the destination alpha value, ignoring the source alpha value.
    const FN_DST: fn(f32, f32) -> f32 = |_, dst| dst;

    /// Returns one minus the source alpha value (`1.0 - src`).
    const FN_ONE_MINUS_SRC: fn(f32, f32) -> f32 = |src, _| 1.0 - src;

    /// Returns one minus the destination alpha value (`1.0 - dst`).
    const FN_ONE_MINUS_DST: fn(f32, f32) -> f32 = |_, dst| 1.0 - dst;

    /// Destination pixels covered by the source are cleared to `0.0`.
    pub const CLEAR: Self = Self::new(Self::FN_ZERO, Self::FN_ZERO);

    /// Destination pixels are replaced with the source pixels.
    pub const SRC: Self = Self::new(Self::FN_ONE, Self::FN_ZERO);

    /// Source pixels are replaced by the destination pixels.
    pub const DST: Self = Self::new(Self::FN_ZERO, Self::FN_ONE);

    /// Source color is placed over the destination color.
    pub const SRC_OVER: Self = Self::new(Self::FN_SRC, Self::FN_ONE_MINUS_SRC);

    /// Destination color is placed over the source color.
    pub const DST_OVER: Self = Self::new(Self::FN_ONE_MINUS_DST, Self::FN_DST);

    /// Source that overlaps the destination replaces the destination.
    pub const SRC_IN: Self = Self::new(Self::FN_DST, Self::FN_ZERO);

    /// Destination that overlaps the source replaces the source.
    pub const DST_IN: Self = Self::new(Self::FN_ZERO, Self::FN_SRC);

    /// Source that does not overlap the destination replaces the destination.
    pub const SRC_OUT: Self = Self::new(Self::FN_ONE_MINUS_DST, Self::FN_ZERO);

    /// Destination that does not overlap the source replaces the source.
    pub const DST_OUT: Self = Self::new(Self::FN_ZERO, Self::FN_ONE_MINUS_SRC);

    /// Source that overlaps the destination is blended with the destination.
    pub const SRC_ATOP: Self = Self::new(Self::FN_DST, Self::FN_ONE_MINUS_SRC);

    /// Destination that overlaps the source is blended with the source.
    pub const DST_ATOP: Self = Self::new(Self::FN_ONE_MINUS_DST, Self::FN_SRC);

    /// Non-overlapping regions of the source and destination are combined.
    pub const XOR: Self = Self::new(Self::FN_ONE_MINUS_DST, Self::FN_ONE_MINUS_SRC);

    /// Source and destination regions are added together.
    pub const PLUS: Self = Self::new(Self::FN_ONE, Self::FN_ONE);
}

impl RgbaBlend for PorterDuff<f32, fn(f32, f32) -> f32> {
    type Channel = f32;

    fn apply(&self, src: Rgba<Self::Channel>, dst: Rgba<Self::Channel>) -> Rgba<Self::Channel> {
        self.blend(src, dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_const_zero() {
        let blend = PorterDuff::<f32, _>::FN_ZERO;
        assert_eq!(blend(0.5, 0.5), 0.0);
        assert_eq!(blend(1.0, 0.0), 0.0);
        assert_eq!(blend(0.0, 1.0), 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_const_one() {
        let blend = PorterDuff::<f32, _>::FN_ONE;
        assert_eq!(blend(0.5, 0.5), 1.0);
        assert_eq!(blend(1.0, 0.0), 1.0);
        assert_eq!(blend(0.0, 1.0), 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_const_src() {
        let blend = PorterDuff::<f32, _>::FN_SRC;
        assert_eq!(blend(0.5, 0.5), 0.5);
        assert_eq!(blend(1.0, 0.0), 1.0);
        assert_eq!(blend(0.0, 1.0), 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_const_dst() {
        let blend = PorterDuff::<f32, _>::FN_DST;
        assert_eq!(blend(0.5, 0.5), 0.5);
        assert_eq!(blend(1.0, 0.0), 0.0);
        assert_eq!(blend(0.0, 1.0), 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_const_one_minus_src() {
        let blend = PorterDuff::<f32, _>::FN_ONE_MINUS_SRC;
        assert_eq!(blend(0.5, 0.5), 0.5);
        assert_eq!(blend(1.0, 0.0), 0.0);
        assert_eq!(blend(0.0, 1.0), 1.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn f32_const_one_minus_dst() {
        let blend = PorterDuff::<f32, _>::FN_ONE_MINUS_DST;
        assert_eq!(blend(0.5, 0.5), 0.5);
        assert_eq!(blend(1.0, 0.0), 1.0);
        assert_eq!(blend(0.0, 1.0), 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn clear() {
        let blend = PorterDuff::<f32, _>::CLEAR;
        let src_c = F32x4Rgba::new(0.5, 0.5, 0.5, 1.0);
        let dst_c = F32x4Rgba::new(0.5, 0.5, 0.5, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::zeroed());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn src() {
        let blend = PorterDuff::<f32, _>::SRC;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, src_c);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn dst() {
        let blend = PorterDuff::<f32, _>::DST;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, dst_c);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn src_over() {
        let blend = PorterDuff::<f32, _>::SRC_OVER;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.1, 0.2, 0.3, 1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn dst_over() {
        let blend = PorterDuff::<f32, _>::DST_OVER;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.4, 0.5, 0.6, 1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn src_in() {
        let blend = PorterDuff::<f32, _>::SRC_IN;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.1, 0.2, 0.3, 1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn dst_in() {
        let blend = PorterDuff::<f32, _>::DST_IN;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.4, 0.5, 0.6, 1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn src_out() {
        let blend = PorterDuff::<f32, _>::SRC_OUT;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn dst_out() {
        let blend = PorterDuff::<f32, _>::DST_OUT;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn src_atop() {
        let blend = PorterDuff::<f32, _>::SRC_ATOP;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.1, 0.2, 0.3, 1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn dst_atop() {
        let blend = PorterDuff::<f32, _>::DST_ATOP;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.4, 0.5, 0.6, 1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn xor() {
        let blend = PorterDuff::<f32, _>::XOR;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn plus() {
        let blend = PorterDuff::<f32, _>::PLUS;
        let src_c = F32x4Rgba::new(0.1, 0.2, 0.3, 1.0);
        let dst_c = F32x4Rgba::new(0.4, 0.5, 0.6, 1.0);
        let result = blend.apply(src_c, dst_c);
        assert_eq!(result, F32x4Rgba::new(0.5, 0.7, 0.900_000_04, 2.0));
    }
}
