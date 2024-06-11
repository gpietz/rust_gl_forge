use anyhow::Result;
use std::path::Path;

pub fn file_size<P: AsRef<Path>>(path: P) -> Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}
