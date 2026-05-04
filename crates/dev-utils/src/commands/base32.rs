use anyhow::{Result, anyhow};
use base32::{Alphabet, encode, decode};

pub fn encode_base32(input: &str) -> Result<()> {
    let encoded = encode(Alphabet::Rfc4648 { padding: true }, input.as_bytes());
    println!("{}", encoded);
    Ok(())
}

pub fn decode_base32(input: &str) -> Result<()> {
    let decoded = decode(Alphabet::Rfc4648 { padding: true }, input)
        .ok_or_else(|| anyhow!("Failed to decode base32"))?;
    
    match String::from_utf8(decoded.clone()) {
        Ok(s) => println!("{}", s),
        Err(_) => {
            use std::io::{self, Write};
            io::stdout().write_all(&decoded)?;
            io::stdout().flush()?;
        }
    }
    Ok(())
}
