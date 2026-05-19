use anyhow::{Result, anyhow};
use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn calculate(text: &str, key: &str, algo: &str) -> Result<()> {
    let result = match algo.to_lowercase().as_str() {
        "md5" => {
            let mut mac = Hmac::<Md5>::new_from_slice(key.as_bytes())
                .map_err(|e| anyhow!("Invalid key length for MD5 HMAC: {}", e))?;
            mac.update(text.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        }
        "sha1" => {
            let mut mac = Hmac::<Sha1>::new_from_slice(key.as_bytes())
                .map_err(|e| anyhow!("Invalid key length for SHA1 HMAC: {}", e))?;
            mac.update(text.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        }
        "sha256" => {
            let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
                .map_err(|e| anyhow!("Invalid key length for SHA256 HMAC: {}", e))?;
            mac.update(text.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        }
        "sha512" => {
            let mut mac = Hmac::<Sha512>::new_from_slice(key.as_bytes())
                .map_err(|e| anyhow!("Invalid key length for SHA512 HMAC: {}", e))?;
            mac.update(text.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        }
        _ => return Err(anyhow!("Unsupported algorithm: {}. Supported: md5, sha1, sha256, sha512", algo)),
    };

    println!("{} {}", Theme::info("Algorithm: "), algo.yellow());
    println!("{}    {}", Theme::info("HMAC:      "), result.bright_white().bold());
    
    Ok(())
}
