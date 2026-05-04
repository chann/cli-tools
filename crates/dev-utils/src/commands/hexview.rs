use anyhow::Result;
use std::fs::File;
use std::io::{Read, BufReader};
use owo_colors::OwoColorize;

pub fn view(input: &str, is_file: bool) -> Result<()> {
    let bytes = if is_file {
        let file = File::open(input)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        buffer
    } else {
        input.as_bytes().to_vec()
    };

    println!("{}", format!("{:<8}  {:<47}  {:<16}", "Offset", "Hex", "ASCII").bold().cyan());
    println!("{}", "-".repeat(75).dimmed());

    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        
        let hex_part: String = chunk.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        
        let ascii_part: String = chunk.iter()
            .map(|&b| {
                if b >= 32 && b <= 126 {
                    (b as char).to_string()
                } else {
                    ".".to_string()
                }
            })
            .collect();

        println!("{:08x}  {:47}  |{}|", offset.green(), hex_part.yellow(), ascii_part.blue());
    }

    Ok(())
}
