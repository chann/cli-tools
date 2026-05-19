use base64::{engine::general_purpose, Engine as _};
use anyhow::{Result, anyhow};
use std::fs;
use std::io::{self, Write};
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn encode(input: &str, is_file: bool, data_uri: bool, url_safe: bool) -> Result<()> {
    let (data, mime, size) = if is_file {
        let content = fs::read(input)?;
        let size = content.len();
        let mime = mime_guess::from_path(input).first_raw().unwrap_or("application/octet-stream");
        (content, Some(mime), size)
    } else {
        (input.as_bytes().to_vec(), None, input.len())
    };
    
    let engine = if url_safe {
        general_purpose::URL_SAFE
    } else {
        general_purpose::STANDARD
    };
    let encoded = engine.encode(&data);
    
    if is_file {
        println!("{}", Theme::header("Base64 Encode Details"));
        println!("{} {}", Theme::info("Input File: "), input.bright_white());
        println!("{} {} bytes", Theme::info("Input Size: "), size.to_string().yellow());
        if let Some(m) = mime {
            println!("{} {}", Theme::info("MIME Type:  "), m.cyan());
        }
        println!();
    }

    if data_uri {
        let uri = if let Some(m) = mime {
            format!("data:{};base64,{}", m, encoded)
        } else {
            format!("data:text/plain;base64,{}", encoded)
        };
        println!("{}", uri.bright_white());
    } else {
        println!("{}", encoded.bright_white());
    }
    
    Ok(())
}

pub fn decode(text: &str, output_file: Option<String>, url_safe: bool) -> Result<()> {
    let engine = if url_safe {
        general_purpose::URL_SAFE
    } else {
        general_purpose::STANDARD
    };
    let decoded_bytes = engine
        .decode(text.trim())
        .map_err(|e| anyhow!("Failed to decode base64: {}", e))?;
    
    if let Some(path) = output_file {
        fs::write(&path, &decoded_bytes)?;
        println!("{}", Theme::success(format!("Decoded data saved to {}", path.bright_white())));
    } else {
        match String::from_utf8(decoded_bytes.clone()) {
            Ok(decoded) => println!("{}", decoded),
            Err(_) => {
                // If not valid UTF-8, write binary to stdout
                println!("{}", Theme::warning("Decoded data is not valid UTF-8, outputting raw binary..."));
                io::stdout().write_all(&decoded_bytes)?;
                io::stdout().flush()?;
            }
        }
    }
    Ok(())
}
