extern crate std;
use alpha_blend::{
    BlendMode, RgbaBlend,
    rgba::{F32x4Rgba, U8x4Rgba},
};
use png::Encoder;
use std::vec::Vec;

fn main() {
    const ALL: [BlendMode; 13] = [
        BlendMode::Clear,
        BlendMode::Source,
        BlendMode::Destination,
        BlendMode::SourceOver,
        BlendMode::DestinationOver,
        BlendMode::SourceIn,
        BlendMode::DestinationIn,
        BlendMode::SourceOut,
        BlendMode::DestinationOut,
        BlendMode::SourceAtop,
        BlendMode::DestinationAtop,
        BlendMode::Xor,
        BlendMode::Plus,
    ];

    // Create an "examples/out" directory for the blended canvases.
    let temp_dir = std::path::Path::new("examples/out");
    std::fs::create_dir_all(temp_dir).unwrap();

    for blend_mode in ALL {
        let blue_square = make_100x100_canvas_with_blue_square_in_bottom_left();
        let red_square = make_100x100_canvas_with_red_square_in_top_right();
        let blended = blend_canvases(&blue_square, &red_square, &blend_mode);

        let rgba8888: Vec<U8x4Rgba> = blended.iter().map(|c| (*c).into()).collect();
        let as_raw_data: &[u8] = bytemuck::cast_slice(&rgba8888);
        let name = format!("blend_{blend_mode:?}.png");

        // Encode the pixel buffer to PNG.
        #[allow(clippy::cast_possible_truncation)]
        let mut encoder = Encoder::new(
            std::fs::File::create(temp_dir.join(&name)).unwrap(),
            100,
            100,
        );
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(as_raw_data).unwrap();
        println!(
            "Wrote blended canvas for {blend_mode:?} to {name}",
            blend_mode = blend_mode,
            name = &name
        );
    }
}

fn make_100x100_canvas_with_blue_square_in_bottom_left() -> Vec<F32x4Rgba> {
    let mut canvas = vec![F32x4Rgba::zeroed(); 100 * 100];
    // Blue square: bottom left, 75x75, overlaps top right red square by 25x25
    for y in 25..100 {
        for x in 0..75 {
            canvas[y * 100 + x] = F32x4Rgba::new(0.0, 0.0, 1.0, 0.5);
        }
    }
    canvas
}

fn make_100x100_canvas_with_red_square_in_top_right() -> Vec<F32x4Rgba> {
    let mut canvas = vec![F32x4Rgba::zeroed(); 100 * 100];
    // Red square: top right, 75x75, overlaps bottom left blue square by 25x25
    for y in 0..75 {
        for x in 25..100 {
            canvas[y * 100 + x] = F32x4Rgba::new(1.0, 0.0, 0.0, 0.5);
        }
    }
    canvas
}

fn blend_canvases(
    src: &[F32x4Rgba],
    dst: &[F32x4Rgba],
    blend: &impl RgbaBlend<Channel = f32>,
) -> Vec<F32x4Rgba> {
    assert_eq!(src.len(), dst.len());
    let mut result = Vec::with_capacity(src.len());
    for (s, d) in src.iter().zip(dst.iter()) {
        result.push(blend.apply(*s, *d));
    }
    result
}
