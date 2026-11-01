use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::util::{default_output_path, ensure_parent};

pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("failed to read {}", path.display()))
}

pub fn write_bytes(path: &Path, data: &[u8]) -> Result<()> {
    ensure_parent(path)?;
    let mut file =
        File::create(path).with_context(|| format!("failed to create {}", path.display()))?;
    file.write_all(data)
        .with_context(|| format!("failed to write {}", path.display()))
}

pub fn file_size(path: &Path) -> Result<u64> {
    let meta = fs::metadata(path).with_context(|| format!("failed to stat {}", path.display()))?;
    Ok(meta.len())
}

pub fn resolve_output(
    input: &Path,
    extension: &str,
    output: Option<&Path>,
    in_place: bool,
) -> Result<PathBuf> {
    if let Some(out) = output {
        return Ok(out.to_path_buf());
    }
    if in_place {
        return Ok(input.to_path_buf());
    }
    Ok(default_output_path(input, extension))
}

pub fn same_path(a: &Path, b: &Path) -> Result<bool> {
    let a_canon = a
        .canonicalize()
        .map_err(|e| anyhow!("failed to canonicalize {}: {e}", a.display()))?;
    let Ok(b_canon) = b.canonicalize() else {
        return Ok(false);
    };
    Ok(a_canon == b_canon)
}
