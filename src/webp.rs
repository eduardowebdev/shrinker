use std::path::Path;

use anyhow::{anyhow, Context, Result};
use image::GenericImageView;

use crate::types::CompressedImage;

/// Decode any supported image and re-encode it as lossless WebP.
pub fn encode_lossless(path: &Path) -> Result<CompressedImage> {
    let img = image::open(path)
        .with_context(|| format!("failed to decode {}", path.display()))?;
    encode_lossless_image(&img)
}

pub fn encode_lossless_image(img: &image::DynamicImage) -> Result<CompressedImage> {
    let (width, height) = img.dimensions();
    if width == 0 || height == 0 {
        return Err(anyhow!("image has zero dimensions"));
    }
    let rgba = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), width, height);
    let memory = encoder.encode_lossless();
    let data: Vec<u8> = (*memory).to_vec();
    Ok(CompressedImage {
        data,
        extension: "webp".to_string(),
        width,
        height,
    })
}
