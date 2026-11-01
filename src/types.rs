use std::path::PathBuf;

use crate::format::Format;

/// Metadata collected from an image on disk.
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub path: PathBuf,
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub bytes: u64,
}

/// Result of a compression run for a single file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionResult {
    pub before: u64,
    pub after: u64,
}

impl CompressionResult {
    pub fn saved_bytes(self) -> u64 {
        self.before.saturating_sub(self.after)
    }

    pub fn saved_percent(self) -> f64 {
        if self.before == 0 {
            return 0.0;
        }
        (self.saved_bytes() as f64 / self.before as f64) * 100.0
    }

    pub fn improvement(self) -> bool {
        self.after < self.before
    }
}

/// Payload produced by a lossless encoder.
#[derive(Debug, Clone)]
pub struct CompressedImage {
    pub data: Vec<u8>,
    pub extension: String,
    pub width: u32,
    pub height: u32,
}
