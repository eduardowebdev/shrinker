use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::encode;
use crate::format::{Format, TargetFormat};
use crate::io;
use crate::types::{CompressedImage, CompressionResult, ImageInfo};

#[derive(Debug, Clone, Default)]
pub struct CompressOptions {
    pub target: Option<TargetFormat>,
    pub in_place: bool,
    pub output: Option<PathBuf>,
}

/// Read basic metadata from an image file.
pub fn inspect(path: &Path) -> Result<ImageInfo> {
    let format = Format::detect_from_path(path)?;
    let bytes = io::file_size(path)?;
    let (width, height) = image::image_dimensions(path)
        .with_context(|| format!("failed to read dimensions of {}", path.display()))?;
    Ok(ImageInfo {
        path: path.to_path_buf(),
        format,
        width,
        height,
        bytes,
    })
}

/// Run the full compression pipeline on a single file.
pub fn compress_file(path: &Path, options: &CompressOptions) -> Result<CompressionResult> {
    let original_size = io::file_size(path)?;
    let format = Format::detect_from_path(path)?;
    if !format.is_supported_lossless_source() {
        return Err(anyhow!("unsupported input format: {}", format.label()));
    }

    let encoded = encode::encode_lossless(path, format, options.target)
        .with_context(|| format!("failed to compress {}", path.display()))?;
    let (payload, extension) = pick_smaller(path, encoded, original_size)?;

    if options.in_place {
        let original_ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();
        if original_ext != extension {
            return Err(anyhow!(
                "cannot change format in-place ({} -> {}); use --output instead",
                original_ext,
                extension
            ));
        }
    }

    let output = io::resolve_output(path, &extension, options.output.as_deref(), options.in_place)?;
    io::write_bytes(&output, &payload)?;
    let final_size = io::file_size(&output)?;

    Ok(CompressionResult {
        before: original_size,
        after: final_size,
    })
}

fn pick_smaller(
    path: &Path,
    encoded: CompressedImage,
    original_size: u64,
) -> Result<(Vec<u8>, String)> {
    if (encoded.data.len() as u64) < original_size {
        return Ok((encoded.data, encoded.extension));
    }
    let original = io::read_bytes(path)?;
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("input has no extension: {}", path.display()))?
        .to_ascii_lowercase();
    Ok((original, ext))
}
