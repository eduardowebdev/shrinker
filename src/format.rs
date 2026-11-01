use std::path::Path;

use anyhow::{anyhow, Result};
use image::ImageFormat;

/// The image format of a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Png,
    Jpeg,
    WebP,
    Gif,
    Bmp,
    Tiff,
    Other,
}

impl Format {
    pub fn from_image_format(fmt: ImageFormat) -> Self {
        match fmt {
            ImageFormat::Png => Format::Png,
            ImageFormat::Jpeg => Format::Jpeg,
            ImageFormat::WebP => Format::WebP,
            ImageFormat::Gif => Format::Gif,
            ImageFormat::Bmp => Format::Bmp,
            ImageFormat::Tiff => Format::Tiff,
            _ => Format::Other,
        }
    }

    pub fn detect_from_path(path: &Path) -> Result<Self> {
        let reader = image::ImageReader::open(path)
            .map_err(|e| anyhow!("could not open {}: {e}", path.display()))?
            .with_guessed_format()
            .map_err(|e| anyhow!("could not guess format for {}: {e}", path.display()))?;
        let fmt = reader
            .format()
            .ok_or_else(|| anyhow!("unknown image format for {}", path.display()))?;
        Ok(Self::from_image_format(fmt))
    }

    pub fn extension(self) -> &'static str {
        match self {
            Format::Png => "png",
            Format::Jpeg => "jpg",
            Format::WebP => "webp",
            Format::Gif => "gif",
            Format::Bmp => "bmp",
            Format::Tiff => "tiff",
            Format::Other => "bin",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Format::Png => "PNG",
            Format::Jpeg => "JPEG",
            Format::WebP => "WebP",
            Format::Gif => "GIF",
            Format::Bmp => "BMP",
            Format::Tiff => "TIFF",
            Format::Other => "unknown",
        }
    }

    /// Formats that we can decode and losslessly re-encode.
    pub fn is_supported_lossless_source(self) -> bool {
        !matches!(self, Format::Other)
    }
}

/// Format we want to write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFormat {
    Png,
    WebP,
}

impl TargetFormat {
    pub fn extension(self) -> &'static str {
        match self {
            TargetFormat::Png => "png",
            TargetFormat::WebP => "webp",
        }
    }
}

pub fn parse_target_format(name: &str) -> Result<Option<TargetFormat>> {
    match name.trim().to_ascii_lowercase().as_str() {
        "auto" => Ok(None),
        "png" => Ok(Some(TargetFormat::Png)),
        "webp" => Ok(Some(TargetFormat::WebP)),
        other => Err(anyhow!(
            "unsupported target format `{other}` (expected: auto, png, webp)"
        )),
    }
}

pub fn choose_target(source: Format) -> TargetFormat {
    match source {
        Format::Png => TargetFormat::Png,
        Format::Jpeg | Format::WebP | Format::Gif | Format::Bmp | Format::Tiff | Format::Other => {
            TargetFormat::WebP
        }
    }
}
