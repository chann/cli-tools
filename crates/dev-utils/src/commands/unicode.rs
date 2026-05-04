use anyhow::Result;
use owo_colors::OwoColorize;
use unicode_names2;

pub fn inspect(text: &str) -> Result<()> {
    println!("{:<4} | {:<10} | {:<8} | {}", 
        "Char".bold(), 
        "Unicode".bold(), 
        "UTF-8".bold(), 
        "Name".bold()
    );
    println!("{}", "-".repeat(60));

    for c in text.chars() {
        let name = unicode_names2::name(c)
            .map(|n| n.to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        let utf8_bytes = c.len_utf8();
        let mut bytes = [0u8; 4];
        c.encode_utf8(&mut bytes);
        let utf8_hex = bytes[..utf8_bytes]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");

        let char_display = if c.is_control() || c.is_whitespace() {
            format!("{:?}", c).dimmed().to_string()
        } else {
            c.to_string().cyan().to_string()
        };

        println!("{:<4} | U+{:04X}     | {:<8} | {}", 
            char_display,
            c as u32,
            utf8_hex,
            name
        );
    }

    Ok(())
}
