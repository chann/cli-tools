use anyhow::{Result, anyhow};

pub fn encode_base58(input: &str) -> Result<()> {
    let encoded = bs58::encode(input.as_bytes()).into_string();
    println!("{}", encoded);
    Ok(())
}

pub fn decode_base58(input: &str) -> Result<()> {
    let decoded = bs58::decode(input)
        .into_vec()
        .map_err(|e| anyhow!("Failed to decode Base58: {}", e))?;
    
    match String::from_utf8(decoded) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            // If it's not valid UTF-8, print as hex
            println!("Decoded data (hex): {}", hex::encode(e.into_bytes()));
        }
    }
    Ok(())
}
