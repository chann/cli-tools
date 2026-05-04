use anyhow::Result;
use owo_colors::OwoColorize;

pub fn convert(input: &str) -> Result<()> {
    let (r, g, b) = if input.starts_with('#') || (input.len() == 6 && input.chars().all(|c| c.is_ascii_hexdigit())) {
        // Hex to RGB
        let hex = input.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color. Use #RRGGBB format.");
        }
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        println!("Hex: {}", input.bold());
        println!("RGB: {}({}, {}, {})", "rgb".bold(), r, g, b);
        (r, g, b)
    } else if input.contains(',') {
        // RGB to Hex
        let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
        if parts.len() != 3 {
            anyhow::bail!("Invalid RGB color. Use R,G,B format.");
        }
        let r: u8 = parts[0].parse()?;
        let g: u8 = parts[1].parse()?;
        let b: u8 = parts[2].parse()?;
        
        println!("RGB: {}({}, {}, {})", "rgb".bold(), r, g, b);
        println!("Hex: {}", format!("#{:02X}{:02X}{:02X}", r, g, b).bold());
        (r, g, b)
    } else {
        anyhow::bail!("Invalid input. Use #RRGGBB or R,G,B format.");
    };

    println!("\n{}", "--- Color Palettes ---".bold().cyan());
    
    // Complementary
    let comp = (255 - r, 255 - g, 255 - b);
    print_color("Complementary", comp);

    // Monochromatic (simpler version)
    print_color("Mono (Dark)", (r / 2, g / 2, b / 2));
    print_color("Mono (Light)", (r + (255 - r) / 2, g + (255 - g) / 2, b + (255 - b) / 2));

    // Simple Analogous (by shifting components)
    print_color("Analogous 1", (r.wrapping_add(30), g, b.wrapping_sub(30)));
    print_color("Analogous 2", (r.wrapping_sub(30), g, b.wrapping_add(30)));
    
    Ok(())
}

fn print_color(label: &str, rgb: (u8, u8, u8)) {
    let hex = format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2);
    println!("{:<15}: {} RGB({}, {}, {})", label, hex.bold(), rgb.0, rgb.1, rgb.2);
}
