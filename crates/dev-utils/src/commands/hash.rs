use sha2::{Sha256, Digest};
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use owo_colors::OwoColorize;

pub fn hash_string(text: &str) -> Result<()> {
    let hash = calculate_string_hash(text);
    println!("{}", hash);
    Ok(())
}

pub fn hash_file(path: &Path) -> Result<()> {
    let hash = calculate_file_hash(path)?;
    println!("{}", hash);
    Ok(())
}

pub fn compare(input: &str, target: &str, is_file: bool) -> Result<()> {
    let input_hash = if is_file {
        calculate_file_hash(Path::new(input))?
    } else {
        calculate_string_hash(input)
    };

    let target_hash = if is_file && Path::new(target).exists() {
        calculate_file_hash(Path::new(target))?
    } else {
        target.to_string()
    };

    println!("Input Hash:  {}", input_hash);
    println!("Target Hash: {}", target_hash);

    if input_hash.to_lowercase() == target_hash.to_lowercase() {
        println!("{}", "MATCH!".bold().green());
    } else {
        println!("{}", "MISMATCH!".bold().red());
    }

    Ok(())
}

fn calculate_string_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    let result = hasher.finalize();
    hex::encode(result)
}

fn calculate_file_hash(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }
    
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
