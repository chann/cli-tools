use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn calculate(text: &str, key: &str) -> Result<()> {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|e| anyhow::anyhow!("Invalid key length: {}", e))?;
    mac.update(text.as_bytes());
    let result = mac.finalize();
    println!("{}", hex::encode(result.into_bytes()));
    Ok(())
}
