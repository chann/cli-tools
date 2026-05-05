use anyhow::Result;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub struct ScanOptions {
    pub duplicates: bool,
    pub empty: bool,
    pub dirs: bool,
    pub links: bool,
    pub min_size: u64,
}

pub fn run(path: &Path, opts: ScanOptions) -> Result<()> {
    println!("{}", Theme::info(format!("Scanning: {} ...", path.display())));

    let mut files_by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    let mut empty_files = Vec::new();
    let mut empty_dirs = Vec::new();
    let mut broken_links = Vec::new();

    for result in WalkBuilder::new(path).build() {
        if let Ok(entry) = result {
            let path = entry.path().to_path_buf();
            
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();
                    if size == 0 {
                        empty_files.push(path);
                    } else if size >= opts.min_size {
                        files_by_size.entry(size).or_default().push(path);
                    }
                }
            } else if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if let Ok(mut read_dir) = fs::read_dir(&path) {
                    if read_dir.next().is_none() {
                        empty_dirs.push(path);
                    }
                }
            } else if entry.file_type().map(|ft| ft.is_symlink()).unwrap_or(false) {
                if fs::read_link(&path).and_then(|target| fs::metadata(path.parent().unwrap().join(target))).is_err() {
                    broken_links.push(path);
                }
            }
        }
    }

    if opts.duplicates {
        let mut duplicates: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for (_size, paths) in files_by_size.iter().filter(|(_, p)| p.len() > 1) {
            for path in paths {
                if let Ok(hash) = hash_file(path) {
                    duplicates.entry(hash).or_default().push(path.clone());
                }
            }
        }

        println!("\n{}", Theme::header("Duplicate Files"));
        let mut found = false;
        
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("Hash"),
            TableFormatter::header_cell("Files"),
        ]);

        for (hash, paths) in duplicates.iter().filter(|(_, p)| p.len() > 1) {
            found = true;
            let paths_str = paths.iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join("\n");
            
            table.add_row(vec![
                TableFormatter::value_cell(&hash[..12]),
                TableFormatter::value_cell(paths_str),
            ]);
        }
        
        if found {
            println!("{}", table);
        } else {
            println!("  {}", Theme::dim("None found"));
        }
    }

    if opts.empty {
        println!("\n{}", Theme::header("Empty Files"));
        if empty_files.is_empty() {
            println!("  {}", Theme::dim("None found"));
        } else {
            for path in empty_files {
                println!("  {}", Theme::warning(path.display().to_string()));
            }
        }
    }

    if opts.dirs {
        println!("\n{}", Theme::header("Empty Directories"));
        if empty_dirs.is_empty() {
            println!("  {}", Theme::dim("None found"));
        } else {
            for path in empty_dirs {
                println!("  {}", Theme::warning(path.display().to_string()));
            }
        }
    }

    if opts.links {
        println!("\n{}", Theme::header("Broken Symlinks"));
        if broken_links.is_empty() {
            println!("  {}", Theme::dim("None found"));
        } else {
            for path in broken_links {
                println!("  {}", Theme::error(path.display().to_string()));
            }
        }
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
