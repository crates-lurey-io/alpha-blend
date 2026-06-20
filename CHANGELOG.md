# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-06-20

### Fixed

- `U8x4Rgba::source_over` alpha channel now uses the correct Porter-Duff SRC_OVER
  formula: `out_a = src_a + dst_a * (1 - src_a)`. Previously used `a*a` which
  produced values too low for partially-transparent sources (#31).

### Changed

- Replaced `source_over_agrees_with_f32_path` test with `source_over_expected_values`
  comparing against hand-computed standard values (the f32 path has the same bug).

## [0.2.0] - 2026-06-20

### Added

- `U8x4Rgba::from_rgb_u32` / `to_rgb_u32` for u32 packed pixel conversion (#31)
- `U8x4Rgba::source_over` — fast integer SourceOver blend (#31)
- `U8x4Rgba::is_transparent` / `is_opaque` helpers (#31)
- `F32x4Rgba::clamp()` — clamp channels to `[0.0, 1.0]` (#31)
- `F32x4Rgba::premultiply()` / `unpremultiply()` — premultiplied alpha support (#31)
- `F32x4Rgba::lerp()` — linear interpolation between colors (#31)
- Named color constants: `TRANSPARENT`, `BLACK`, `WHITE` on both `U8x4Rgba` and `F32x4Rgba` (#31)
- `Eq + Hash` impls for `U8x4Rgba` (enables `HashMap`/`HashSet` keys) (#31)
- `From<[C;4]>`, `From<(C,C,C,C)>`, and their inverse conversions on `Rgba<C>` (#31)
- `AsRef<[C]>` / `AsMut<[C]>` on `Rgba<C>` (#31)
- `Display` impls for `U8x4Rgba` and `F32x4Rgba` (#31)
- `Default` impl for `BlendMode` (returns `SourceOver`) (#31)
- `Hash` derive on `BlendMode` (#31)
- `Debug`, `Clone`, `Copy` derives on `PorterDuff<C, F>` (#31)
- `PorterDuff::new` is now `pub const` (#31)
- `RgbaBlend::apply_slice` for batch blending with autovectorization potential (#31)
- Size-assert guarded transmutes in `F32x4` ↔ `F32x4Rgba` conversions (#31)
- CI publish workflow (tag-based, OIDC publishing to crates.io) (#31)
- Semver-checks and MSRV targets in Justfile (#31)

### Changed

- **Breaking**: `Rgba<C>` field doc comments are now correct — `r` is Red, `g` is Green,
  `b` is Blue, `a` is Alpha (previously inverted) (#31)
- `BlendMode::apply` now dispatches to `PorterDuff` directly instead of through
  the private `as_rgba_blend_f32` helper (#31)
- Crate-level docs now document the straight-alpha assumption (#31)
- `BlendMode::Plus` docs note that it can produce values > 1.0 (#31)
- `RgbaBlend` trait docs note that only `f32` is currently supported (#31)

### Internal

- Added `rust-version = "1.85"` and expanded rust lint configuration (#31)

## [0.1.2] - 2025-07-19

### Added

- Added an example in `lib.rs`

### Changed

- Tweaked the `## Features` list in `lib.rs`

## [0.1.1] - 2025-07-18

### Changed

- A `!compile_error` if neither `std` or `libm` is enabled

## [0.1.0] - 2025-07-18

### Added

- Initial release
