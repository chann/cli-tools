use base64::{engine::general_purpose, Engine as _};
use anyhow::{Result, anyhow};
use std::fs;
use std::io::{self, Write};

pub fn encode(input: &str, is_file: bool, data_uri: bool) -> Result<()> {
    let (data, mime) = if is_file {
        let content = fs::read(input)?;
        let mime = mime_guess::from_path(input).first_raw().unwrap_or("application/octet-stream");
        (content, Some(mime))
    } else {
        (input.as_bytes().to_vec(), None)
    };
    
    let encoded = general_purpose::STANDARD.encode(data);
    
    if data_uri {
        if let Some(m) = mime {
            println!("data:{};base64,{}", m, encoded);
        } else {
            println!("data:text/plain;base64,{}", encoded);
        }
    } else {
        println!("{}", encoded);
    }
    Ok(())
}

pub fn decode(text: &str, output_file: Option<String>) -> Result<()> {
    let decoded_bytes = general_purpose::STANDARD
        .decode(text)
        .map_err(|e| anyhow!("Failed to decode base64: {}", e))?;
    
    if let Some(path) = output_file {
        fs::write(path, decoded_bytes)?;
    } else {
        match String::from_utf8(decoded_bytes.clone()) {
            Ok(decoded) => println!("{}", decoded),
            Err(_) => {
                // If not valid UTF-8, write binary to stdout
                io::stdout().write_all(&decoded_bytes)?;
                io::stdout().flush()?;
            }
        }
    }
    Ok(())
}
