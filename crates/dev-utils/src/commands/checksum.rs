use anyhow::{Result, anyhow};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};
use std::fs::File;
use std::io::Read;

pub fn calculate(input: &str, algo: &str, is_file: bool) -> Result<()> {
    let result = match algo.to_lowercase().as_str() {
        "md5" => hash::<Md5>(input, is_file)?,
        "sha1" => hash::<Sha1>(input, is_file)?,
        "sha256" => hash::<Sha256>(input, is_file)?,
        "sha512" => hash::<Sha512>(input, is_file)?,
        _ => return Err(anyhow!("Unsupported algorithm: {}. Supported: md5, sha1, sha256, sha512", algo)),
    };

    println!("{}", result);
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
