use anyhow::{Result, anyhow};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use owo_colors::OwoColorize;
use cli_core::ui::Theme;

pub fn hash_string(text: &str, algo: &str) -> Result<()> {
    let hash = calculate_hash::<DigestWrapper>(text.as_bytes(), algo)?;
    println!("{}", hash.bright_white().bold());
    Ok(())
}

pub fn hash_file(path: &Path, algo: &str) -> Result<()> {
    let hash = calculate_file_hash_internal(path, algo)?;
    println!("{}", hash.bright_white().bold());
    Ok(())
}

pub fn compare(input: &str, target: &str, algo: &str, is_file: bool) -> Result<()> {
    let input_hash = if is_file {
        calculate_file_hash_internal(Path::new(input), algo)?
    } else {
        let mut hasher = create_hasher(algo)?;
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    };

    let target_hash = if is_file && Path::new(target).exists() {
        calculate_file_hash_internal(Path::new(target), algo)?
    } else {
        target.to_string()
    };

    println!("{} {}", Theme::info("Algorithm: "), algo.yellow());
    println!("{}  {}", Theme::info("Input Hash: "), input_hash.bright_white());
    println!("{} {}", Theme::info("Target Hash:"), target_hash.bright_white());

    if input_hash.to_lowercase() == target_hash.to_lowercase() {
        println!("\n{}", Theme::success("MATCH!"));
    } else {
        println!("\n{}", Theme::error("MISMATCH!"));
    }

    Ok(())
}

fn calculate_file_hash_internal(path: &Path, algo: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = create_hasher(algo)?;
    let mut buffer = [0; 8192];
    
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }
    
    Ok(hex::encode(hasher.finalize()))
}

enum DigestWrapper {
    Md5(Md5),
    Sha1(Sha1),
    Sha256(Sha256),
    Sha512(Sha512),
}

impl DigestWrapper {
    fn update(&mut self, data: &[u8]) {
        match self {
            Self::Md5(d) => d.update(data),
            Self::Sha1(d) => d.update(data),
            Self::Sha256(d) => d.update(data),
            Self::Sha512(d) => d.update(data),
        }
    }

    fn finalize(self) -> Vec<u8> {
        match self {
            Self::Md5(d) => d.finalize().to_vec(),
            Self::Sha1(d) => d.finalize().to_vec(),
            Self::Sha256(d) => d.finalize().to_vec(),
            Self::Sha512(d) => d.finalize().to_vec(),
        }
    }
}

fn create_hasher(algo: &str) -> Result<DigestWrapper> {
    match algo.to_lowercase().as_str() {
        "md5" => Ok(DigestWrapper::Md5(Md5::new())),
        "sha1" => Ok(DigestWrapper::Sha1(Sha1::new())),
        "sha256" => Ok(DigestWrapper::Sha256(Sha256::new())),
        "sha512" => Ok(DigestWrapper::Sha512(Sha512::new())),
        _ => Err(anyhow!("Unsupported algorithm: {}. Supported: md5, sha1, sha256, sha512", algo)),
    }
}

fn calculate_hash<T>(data: &[u8], algo: &str) -> Result<String> {
    let mut hasher = create_hasher(algo)?;
    hasher.update(data);
    Ok(hex::encode(hasher.finalize()))
}
