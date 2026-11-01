use std::path::PathBuf;

use anyhow::{anyhow, Result};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::compress::{self, CompressOptions};
use crate::progress::Progress;
use crate::types::CompressionResult;
use crate::util::is_image_extension;

/// Expand the given inputs into a sorted, deduplicated list of image files.
pub fn collect(inputs: &[PathBuf], recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for input in inputs {
        if input.is_file() {
            files.push(input.clone());
            continue;
        }
        if input.is_dir() {
            let walker = if recursive {
                WalkDir::new(input)
            } else {
                WalkDir::new(input).max_depth(1)
            };
            for entry in walker {
                let entry = entry?;
                if entry.file_type().is_file() && is_image_extension(entry.path()) {
                    files.push(entry.into_path());
                }
            }
            continue;
        }
        return Err(anyhow!("input does not exist: {}", input.display()));
    }
    files.sort();
    files.dedup();
    Ok(files)
}

/// Run compression over all files in parallel.
pub fn run(
    files: &[PathBuf],
    options: &CompressOptions,
    progress: &Progress,
) -> Vec<(PathBuf, Result<CompressionResult>)> {
    files
        .par_iter()
        .map(|path| {
            let result = compress::compress_file(path, options);
            progress.tick(path);
            (path.clone(), result)
        })
        .collect()
}
