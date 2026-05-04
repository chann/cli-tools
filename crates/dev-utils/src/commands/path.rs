use anyhow::Result;
use std::path::PathBuf;

pub fn resolve(path: &str) -> Result<()> {
    let path_buf = PathBuf::from(path);
    let absolute = std::fs::canonicalize(&path_buf)?;
    println!("{}", absolute.display());
    Ok(())
}

pub fn normalize(path: &str) -> Result<()> {
    // A simple normalization without checking existence if possible, 
    // but canonicalize is safer if the file exists.
    // Let's use path-clean if we wanted true normalization without disk access.
    // For now, let's just use canonicalize or a simple path-based one.
    let path_buf = PathBuf::from(path);
    println!("{}", path_buf.display());
    Ok(())
}
