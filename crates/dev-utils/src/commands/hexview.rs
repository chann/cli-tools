use anyhow::Result;
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn view(input: &str, is_file: bool) -> Result<()> {
    let bytes = if is_file {
        std::fs::read(input)?
    } else {
        input.as_bytes().to_vec()
    };

    println!("{}", Theme::header(format!("Hex View ({} bytes)", bytes.len())));
    println!("{}", Theme::dim("Offset    00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F  ASCII"));
    println!("{}", Theme::dim("--------- -----------------------  -----------------------  ----------------"));

    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        print!("{:08x}  ", offset.dimmed());

        // Hex part
        for (j, byte) in chunk.iter().enumerate() {
            let hex_str = format!("{:02x}", byte);
            let colored_hex = match *byte {
                0 => hex_str.dimmed().to_string(),
                32..=126 => hex_str.green().to_string(),
                _ => hex_str.yellow().to_string(),
            };
            print!("{} ", colored_hex);
            if j == 7 {
                print!(" ");
            }
        }

        // Padding for hex part if chunk < 16
        if chunk.len() < 16 {
            for j in chunk.len()..16 {
                print!("   ");
                if j == 7 {
                    print!(" ");
                }
            }
        }

        print!(" ");

        // ASCII part
        for byte in chunk {
            if (32..=126).contains(byte) {
                print!("{}", (*byte as char).green());
            } else {
                print!("{}", ".".dimmed());
            }
        }

        println!();

        // Limit output for very large files to avoid flooding the terminal
        if i >= 64 {
            println!("{}", Theme::info("... output truncated ..."));
            break;
        }
    }

    Ok(())
}
