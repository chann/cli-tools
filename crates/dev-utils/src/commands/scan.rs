use anyhow::Result;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use owo_colors::OwoColorize;

pub fn run(path: &Path, min_size: u64) -> Result<()> {
    println!("Scanning for duplicate files in {} (min size: {} bytes, respecting .gitignore)...", path.display().cyan(), min_size);

    let mut files_by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for result in WalkBuilder::new(path).build() {
        if let Ok(entry) = result {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();
                    if size >= min_size {
                        files_by_size.entry(size).or_default().push(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    let mut duplicates: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (_size, paths) in files_by_size.iter().filter(|(_, p)| p.len() > 1) {
        for path in paths {
            if let Ok(hash) = hash_file(path) {
                duplicates.entry(hash).or_default().push(path.clone());
            }
        }
    }

    let mut found = false;
    for (hash, paths) in duplicates.iter().filter(|(_, p)| p.len() > 1) {
        found = true;
        println!("\n{} {}", "Duplicate hash:".bold().yellow(), hash.dimmed());
        for path in paths {
            println!("  - {}", path.display());
        }
    }

    if !found {
        println!("{}", "No duplicate files found.".green());
    }

    Ok(())
}

fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}
