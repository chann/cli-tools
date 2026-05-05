use anyhow::{Result, anyhow};
use base32::{Alphabet, encode, decode};
use std::fs;
use std::io::{self, Write};
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn encode_base32(input: &str, is_file: bool) -> Result<()> {
    let (data, size) = if is_file {
        let content = fs::read(input)?;
        let size = content.len();
        (content, size)
    } else {
        (input.as_bytes().to_vec(), input.len())
    };

    if is_file {
        println!("{}", Theme::header("Base32 Encode Details"));
        println!("{} {}", Theme::info("Input File: "), input.bright_white());
        println!("{} {} bytes", Theme::info("Input Size: "), size.to_string().yellow());
        println!();
    }

    let encoded = encode(Alphabet::Rfc4648 { padding: true }, &data);
    println!("{}", encoded.bright_white());
    Ok(())
}

pub fn decode_base32(input: &str, output_file: Option<String>) -> Result<()> {
    let decoded = decode(Alphabet::Rfc4648 { padding: true }, input)
        .ok_or_else(|| anyhow!("Failed to decode base32. Ensure input is valid RFC4648 base32."))?;
    
    if let Some(path) = output_file {
        fs::write(&path, &decoded)?;
        println!("{}", Theme::success(format!("Decoded data saved to {}", path.bright_white())));
    } else {
        match String::from_utf8(decoded.clone()) {
            Ok(s) => println!("{}", s),
            Err(_) => {
                println!("{}", Theme::warning("Decoded data is not valid UTF-8, outputting raw binary..."));
                io::stdout().write_all(&decoded)?;
                io::stdout().flush()?;
            }
        }
    }
    Ok(())
}
