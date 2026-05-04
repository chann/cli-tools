use urlencoding::{encode, decode};
use anyhow::Result;

pub fn encode_url(text: &str) -> Result<()> {
    println!("{}", encode(text));
    Ok(())
}

pub fn decode_url(text: &str) -> Result<()> {
    println!("{}", decode(text)?);
    Ok(())
}
