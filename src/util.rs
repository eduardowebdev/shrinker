use std::path::{Path, PathBuf};

pub fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut value = bytes as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.2} {}", value, UNITS[unit])
    }
}

pub fn default_output_path(input: &Path, new_extension: &str) -> PathBuf {
    let stem = input
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "output".to_string());
    let filename = format!("{stem}.shrink.{new_extension}");
    let mut out = input.to_path_buf();
    out.set_file_name(filename);
    out
}

pub fn with_extension(path: &Path, ext: &str) -> PathBuf {
    let mut p = path.to_path_buf();
    p.set_extension(ext);
    p
}

pub fn is_image_extension(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp" | "tif" | "tiff"
    )
}

pub fn ensure_parent(path: &Path) -> std::io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if parent.as_os_str().is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(parent)
}
