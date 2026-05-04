use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde_json::Value;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn inspect(token: &str) -> Result<()> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        anyhow::bail!("Invalid JWT token format. Must have at least header and payload.");
    }

    println!("--- Header ---");
    let header_json = decode_part(parts[0])?;
    println!("{}", serde_json::to_string_pretty(&header_json)?);

    println!("\n--- Payload ---");
    let payload_json = decode_part(parts[1])?;
    println!("{}", serde_json::to_string_pretty(&payload_json)?);

    if parts.len() > 2 {
        println!("\n--- Signature ---");
        println!("[Encoded Signature]");
    }

    Ok(())
}

pub fn sign(payload: &str, secret: &str) -> Result<()> {
    let header = serde_json::json!({
        "alg": "HS256",
        "typ": "JWT"
    });

    let payload_value: Value = serde_json::from_str(payload)?;

    let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header)?);
    let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload_value)?);

    let message = format!("{}.{}", header_b64, payload_b64);
    
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature_b64 = URL_SAFE_NO_PAD.encode(result.into_bytes());

    println!("{}.{}", message, signature_b64);
    Ok(())
}

fn decode_part(part: &str) -> Result<Value> {
    let decoded = URL_SAFE_NO_PAD.decode(part)?;
    let json: Value = serde_json::from_slice(&decoded)?;
    Ok(json)
}
