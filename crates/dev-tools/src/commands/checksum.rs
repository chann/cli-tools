use anyhow::{Result, anyhow};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};
use std::fs::File;
use std::io::Read;
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn calculate(input: &str, algo: &str, is_file: bool) -> Result<()> {
    let result = match algo.to_lowercase().as_str() {
        "md5" => hash::<Md5>(input, is_file)?,
        "sha1" => hash::<Sha1>(input, is_file)?,
        "sha256" => hash::<Sha256>(input, is_file)?,
        "sha512" => hash::<Sha512>(input, is_file)?,
        _ => return Err(anyhow!("Unsupported algorithm: {}. Supported: md5, sha1, sha256, sha512", algo)),
    };

    println!("{} {}", Theme::info("Algorithm: "), algo.yellow());
    println!("{}    {}", Theme::info("Source:    "), if is_file { "File".cyan() } else { "Text".cyan() });
    println!("{}    {}", Theme::info("Hash:      "), result.bright_white().bold());
    
    Ok(())
}

pub fn verify(input: &str, expected_hash: &str, algo: &str, is_file: bool) -> Result<()> {
    let actual_hash = match algo.to_lowercase().as_str() {
        "md5" => hash::<Md5>(input, is_file)?,
        "sha1" => hash::<Sha1>(input, is_file)?,
        "sha256" => hash::<Sha256>(input, is_file)?,
        "sha512" => hash::<Sha512>(input, is_file)?,
        _ => return Err(anyhow!("Unsupported algorithm: {}. Supported: md5, sha1, sha256, sha512", algo)),
    };

    println!("{} {}", Theme::info("Algorithm: "), algo.yellow());
    println!("{}    {}", Theme::info("Source:    "), if is_file { "File".cyan() } else { "Text".cyan() });
    println!("{}    {}", Theme::info("Expected:  "), expected_hash.dimmed());
    println!("{}    {}", Theme::info("Actual:    "), actual_hash.bright_white());

    if actual_hash.to_lowercase() == expected_hash.to_lowercase() {
        println!("\n{}", Theme::success("Verification successful! The hashes match."));
    } else {
        println!("\n{}", Theme::error("Verification failed! The hashes do NOT match."));
        anyhow::bail!("Checksum verification failed");
    }
    
    Ok(())
}

fn hash<D: Digest>(input: &str, is_file: bool) -> Result<String> {
    let mut hasher = D::new();
    if is_file {
        let mut file = File::open(input)?;
        let mut buffer = [0u8; 8192];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
    } else {
        hasher.update(input.as_bytes());
    }
    Ok(hex::encode(hasher.finalize()))
}
