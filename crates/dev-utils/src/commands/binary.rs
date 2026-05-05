use anyhow::Result;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use owo_colors::OwoColorize;

pub fn to_binary(input: &str) -> Result<()> {
    println!("{}", Theme::header("Text to Binary Conversion"));
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Char"),
        TableFormatter::header_cell("Hex"),
        TableFormatter::header_cell("Decimal"),
        TableFormatter::header_cell("Binary"),
    ]);

    for b in input.as_bytes() {
        table.add_row(vec![
            TableFormatter::value_cell(format!("'{}'", (*b as char).to_string().cyan())),
            TableFormatter::value_cell(format!("0x{:02X}", b).yellow()),
            TableFormatter::value_cell(b.to_string().green()),
            TableFormatter::value_cell(format!("{:08b}", b).bright_white().bold()),
        ]);
    }

    println!("{}", table);
    
    let result: String = input.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join(" ");
    
    println!("\n{} {}", Theme::info("Full Binary:"), result.bright_white());
    
    Ok(())
}

pub fn from_binary(input: &str) -> Result<()> {
    let clean_input = input.replace(' ', "");
    if clean_input.len() % 8 != 0 {
        anyhow::bail!("Invalid binary string length. Must be a multiple of 8 (each byte is 8 bits).");
    }
    
    let mut bytes = Vec::new();
    for i in (0..clean_input.len()).step_by(8) {
        let byte_str = &clean_input[i..i+8];
        let byte = u8::from_str_radix(byte_str, 2)
            .map_err(|_| anyhow::anyhow!("Invalid binary digit at {}", byte_str))?;
        bytes.push(byte);
    }
    
    let text = String::from_utf8_lossy(&bytes);
    
    println!("{}", Theme::header("Binary to Text Conversion"));
    println!("{} {}", Theme::info("Input:  "), input.bright_white());
    println!("{} {}", Theme::success("Result: "), text.bright_green().bold());
    
    Ok(())
}
