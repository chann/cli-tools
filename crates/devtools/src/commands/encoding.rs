use anyhow::{Result, anyhow};
use cli_core::ui::Theme;
use owo_colors::OwoColorize;
use std::fs;
use std::io::{self, Write};

pub fn encode_data<F>(input: &str, is_file: bool, encode_fn: F, title: &str) -> Result<()>
where
    F: FnOnce(&[u8]) -> String,
{
    let (data, size) = if is_file {
        let content = fs::read(input)?;
        let size = content.len();
        (content, size)
    } else {
        (input.as_bytes().to_vec(), input.len())
    };

    if is_file {
        println!("{}", Theme::header(&format!("{} Encode Details", title)));
        println!("{} {}", Theme::info("Input File: "), input.bright_white());
        println!("{} {} bytes", Theme::info("Input Size: "), size.to_string().yellow());
        println!();
    }

    let encoded = encode_fn(&data);
    println!("{}", encoded.bright_white());
    Ok(())
}

pub fn decode_data<F>(input: &str, output_file: Option<String>, decode_fn: F, error_msg: &str) -> Result<()>
where
    F: FnOnce(&str) -> Option<Vec<u8>>,
{
    let decoded = decode_fn(input.trim())
        .ok_or_else(|| anyhow!("{}", error_msg))?;
    
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
