use std::path::Path;

use anyhow::Result;
use image::GenericImageView;

use crate::format::{choose_target, Format, TargetFormat};
use crate::png;
use crate::types::CompressedImage;
use crate::webp;

/// Re-encode an image losslessly, honoring an explicit target format if given.
pub fn encode_lossless(
    path: &Path,
    source: Format,
    target: Option<TargetFormat>,
) -> Result<CompressedImage> {
    let target = target.unwrap_or_else(|| choose_target(source));
    match (source, target) {
        (Format::Png, TargetFormat::Png) => png::optimize(path),
        (_, TargetFormat::Png) => encode_to_png(path),
        (_, TargetFormat::WebP) => webp::encode_lossless(path),
    }
}

fn encode_to_png(path: &Path) -> Result<CompressedImage> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let mut bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);
    img.write_to(&mut cursor, image::ImageFormat::Png)?;
    let optimized = png::optimize_bytes(&bytes)?;
    Ok(CompressedImage {
        data: optimized,
        extension: "png".to_string(),
        width,
        height,
    })
}
