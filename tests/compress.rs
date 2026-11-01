use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use image::{Rgba, RgbaImage};
use tempfile::TempDir;

use shrinker::compress::{self, CompressOptions};
use shrinker::format::TargetFormat;

fn make_png(dir: &Path, name: &str, w: u32, h: u32) -> Result<PathBuf, Box<dyn Error>> {
    let path = dir.join(name);
    let mut img = RgbaImage::new(w, h);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba([
            (x % 256) as u8,
            (y % 256) as u8,
            ((x + y) % 256) as u8,
            255,
        ]);
    }
    img.save(&path)?;
    Ok(path)
}

#[test]
fn png_in_place_does_not_grow() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let path = make_png(dir.path(), "sample.png", 64, 64)?;
    let before = fs::metadata(&path)?.len();
    let options = CompressOptions {
        target: Some(TargetFormat::Png),
        in_place: true,
        output: None,
    };
    let result = compress::compress_file(&path, &options)?;
    assert_eq!(result.before, before);
    assert!(result.after > 0);
    assert!(result.after <= before);
    Ok(())
}

#[test]
fn writes_to_custom_output() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let path = make_png(dir.path(), "sample.png", 32, 32)?;
    let out = dir.path().join("out.png");
    let options = CompressOptions {
        target: Some(TargetFormat::Png),
        in_place: false,
        output: Some(out.clone()),
    };
    let result = compress::compress_file(&path, &options)?;
    assert!(out.exists());
    assert_eq!(result.after, fs::metadata(&out)?.len());
    Ok(())
}

#[test]
fn rejects_format_change_in_place() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let path = make_png(dir.path(), "sample.png", 16, 16)?;
    let options = CompressOptions {
        target: Some(TargetFormat::WebP),
        in_place: true,
        output: None,
    };
    let err = compress::compress_file(&path, &options).unwrap_err();
    assert!(err.to_string().contains("cannot change format in-place"));
    Ok(())
}

#[test]
fn inspect_reports_dimensions() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let path = make_png(dir.path(), "meta.png", 12, 7)?;
    let info = compress::inspect(&path)?;
    assert_eq!(info.width, 12);
    assert_eq!(info.height, 7);
    assert!(info.bytes > 0);
    Ok(())
}
