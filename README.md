# alpha-blend

Alpha blending and compositing in (optionally) zero-dependency Rust.

[![Test](https://github.com/crates-lurey-io/alpha-blend/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/alpha-blend/actions/workflows/test.yml)
[![Docs](https://github.com/crates-lurey-io/alpha-blend/actions/workflows/docs.yml/badge.svg)](https://github.com/crates-lurey-io/alpha-blend/actions/workflows/docs.yml)
[![Crates.io Version](https://img.shields.io/crates/v/alpha-blend)](https://crates.io/crates/alpha-blend)
[![codecov](https://codecov.io/gh/crates-lurey-io/alpha-blend/graph/badge.svg?token=Z3VUWA3WYY)](https://codecov.io/gh/crates-lurey-io/alpha-blend)

## Examples

```sh
cargo run --example porter-duff --features bytemuck
```

### `BlendMode::Clear`

![](examples/out/blend_Clear.png)

### `BlendMode::Source`

![](examples/out/blend_Source.png)

### `BlendMode::Destination`

![](examples/out/blend_Destination.png)

### `BlendMode::SourceOver`

![](examples/out/blend_SourceOver.png)

### `BlendMode::DestinationOver`

![](examples/out/blend_DestinationOver.png)

### `BlendMode::SourceIn`

![](examples/out/blend_SourceIn.png)

### `BlendMode::DestinationIn`

![](examples/out/blend_DestinationIn.png)

### `BlendMode::SourceOut`

![](examples/out/blend_SourceOut.png)

### `BlendMode::DestinationOut`

![](examples/out/blend_DestinationOut.png)

### `BlendMode::SourceAtop`

![](examples/out/blend_SourceAtop.png)

### `BlendMode::DestinationAtop`

![](examples/out/blend_DestinationAtop.png)

### `BlendMode::Xor`

![](examples/out/blend_Xor.png)

### `BlendMode::Plus`

![](examples/out/blend_Plus.png)

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just coverage` to generate and preview code coverage.
- `cargo just doc` to generate and preview docs.
- `cargo just test` to run tests.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).
