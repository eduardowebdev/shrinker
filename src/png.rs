use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use oxipng::{Options, StripChunks};

use crate::types::CompressedImage;

/// Optimize a PNG file losslessly using oxipng.
pub fn optimize(path: &Path) -> Result<CompressedImage> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let optimized = optimize_bytes(&bytes)?;
    let (width, height) = read_dimensions(&bytes)?;
    Ok(CompressedImage {
        data: optimized,
        extension: "png".to_string(),
        width,
        height,
    })
}

/// Optimize PNG bytes losslessly (no metadata stripping beyond safe).
pub fn optimize_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut options = Options::max_compression();
    options.strip = StripChunks::Safe;
    oxipng::optimize_from_memory(bytes, &options)
        .map_err(|e| anyhow!("oxipng failed: {e}"))
}

fn read_dimensions(bytes: &[u8]) -> Result<(u32, u32)> {
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| anyhow!("invalid png: {e}"))?;
    reader
        .into_dimensions()
        .map_err(|e| anyhow!("could not read png dimensions: {e}"))
}
