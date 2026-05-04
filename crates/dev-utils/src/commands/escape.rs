use anyhow::Result;
use html_escape::{encode_safe, decode_html_entities};

pub fn html_escape(text: &str) -> Result<()> {
    println!("{}", encode_safe(text));
    Ok(())
}

pub fn html_unescape(text: &str) -> Result<()> {
    println!("{}", decode_html_entities(text));
    Ok(())
}

pub fn string_escape(text: &str) -> Result<()> {
    println!("{}", text.escape_debug());
    Ok(())
}
