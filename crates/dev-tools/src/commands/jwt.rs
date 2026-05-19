use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde_json::Value;
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use owo_colors::OwoColorize;
use chrono::{DateTime, Utc, Local};

pub fn inspect(token: &str, secret: Option<&str>) -> Result<()> {
    let header = decode_header(token)?;
    println!("{}", "--- Header ---".bold().cyan());
    println!("{}", serde_json::to_string_pretty(&header)?);

    let mut validation = Validation::new(header.alg);
    validation.validate_exp = false; // We want to see the payload even if expired
    validation.validate_aud = false;
    validation.required_spec_claims.clear();

    let decoding_key = if let Some(s) = secret {
        match header.alg {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                DecodingKey::from_secret(s.as_bytes())
            }
            _ => DecodingKey::from_secret(&[]), // Placeholder for other algs
        }
    } else {
        DecodingKey::from_secret(&[])
    };

    let token_data = decode::<Value>(token, &decoding_key, &validation);

    match token_data {
        Ok(data) => {
            println!("\n{}", "--- Payload ---".bold().green());
            println!("{}", serde_json::to_string_pretty(&data.claims)?);
            
            if secret.is_some() {
                println!("\n{}", "✓ Signature Verified".green().bold());
            }

            // Human readable timestamps
            if let Some(claims) = data.claims.as_object() {
                let mut ts_found = false;
                for (key, label) in [("exp", "Expires"), ("iat", "Issued At"), ("nbf", "Not Before")] {
                    if let Some(ts) = claims.get(key).and_then(|v| v.as_i64()) {
                        if !ts_found {
                            println!("\n{}", "--- Timestamps ---".bold().yellow());
                            ts_found = true;
                        }
                        let dt = DateTime::<Utc>::from_timestamp(ts, 0).map(|d| d.with_timezone(&Local));
                        if let Some(dt) = dt {
                            println!("{:<12}: {}", label, dt.format("%Y-%m-%d %H:%M:%S %Z"));
                        }
                    }
                }
            }
        }
        Err(e) => {
            if secret.is_some() {
                println!("\n{}", format!("✗ Signature Verification Failed: {}", e).red().bold());
            }
            
            // Still try to show payload even if verification fails or no secret provided
            println!("\n{}", "--- Payload (Unverified) ---".bold().yellow());
            let parts: Vec<&str> = token.split('.').collect();
            if parts.len() > 1 {
                let decoded = URL_SAFE_NO_PAD.decode(parts[1])?;
                let json: Value = serde_json::from_slice(&decoded)?;
                println!("{}", serde_json::to_string_pretty(&json)?);
            }
        }
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
    
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature_b64 = URL_SAFE_NO_PAD.encode(result.into_bytes());

    println!("{}.{}", message, signature_b64);
    Ok(())
}

