use std::io::Cursor;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ExtendedColorType, GenericImageView, ImageEncoder};

use crate::types::CompressedImage;

/// Decode any supported image and re-encode it as lossless WebP.
pub fn encode_lossless(path: &Path) -> Result<CompressedImage> {
    let img = image::open(path).with_context(|| format!("failed to decode {}", path.display()))?;
    encode_lossless_image(&img)
}

pub fn encode_lossless_image(img: &DynamicImage) -> Result<CompressedImage> {
    let (width, height) = img.dimensions();
    if width == 0 || height == 0 {
        return Err(anyhow!("image has zero dimensions"));
    }
    let rgba = img.to_rgba8();
    let mut buf: Vec<u8> = Vec::new();
    WebPEncoder::new_lossless(Cursor::new(&mut buf))
        .write_image(rgba.as_raw(), width, height, ExtendedColorType::Rgba8)
        .map_err(|e| anyhow!("webp encode failed: {e}"))?;
    Ok(CompressedImage {
        data: buf,
        extension: "webp".to_string(),
        width,
        height,
    })
}
