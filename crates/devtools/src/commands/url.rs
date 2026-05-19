use urlencoding::{encode, decode};
use anyhow::Result;
use cli_core::ui::Theme;

pub fn encode_url(text: &str) -> Result<()> {
    println!("\n{}", Theme::header("URL Encoded"));
    println!("{}", Theme::highlight(encode(text)));
    Ok(())
}

pub fn decode_url(text: &str) -> Result<()> {
    println!("\n{}", Theme::header("URL Decoded"));
    println!("{}", Theme::highlight(decode(text)?));
    Ok(())
}
