#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("Either the 'std' or 'libm' feature must be enabled for alpha-blend.");

/// Implements rounding for `f32` values.
///
/// If the `std` feature is enabled, it uses `f32::round`, otherwise it uses `libm::roundf`.
pub fn round(f: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::round(f);

    #[cfg(not(feature = "std"))]
    return libm::roundf(f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn round_up() {
        assert_eq!(round(1.5), 2.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn round_down() {
        assert_eq!(round(1.4), 1.0);
    }
}
