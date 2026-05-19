use anyhow::Result;
use owo_colors::OwoColorize;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

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
        (r, g, b)
    } else {
        anyhow::bail!("Invalid input. Use #RRGGBB or R,G,B format.");
    };

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    let (h, s, l) = rgb_to_hsl(r, g, b);

    println!("{}", Theme::header(format!("--- Color Information: {} ---", hex)));
    
    let mut info_table = TableFormatter::create_table();
    info_table.add_row(vec![TableFormatter::header_cell("Hex"), TableFormatter::value_cell(&hex)]);
    info_table.add_row(vec![TableFormatter::header_cell("RGB"), TableFormatter::value_cell(format!("rgb({}, {}, {})", r, g, b))]);
    info_table.add_row(vec![TableFormatter::header_cell("HSL"), TableFormatter::value_cell(format!("hsl({:.0}°, {:.1}%, {:.1}%)", h, s * 100.0, l * 100.0))]);
    info_table.add_row(vec![
        TableFormatter::header_cell("Preview"),
        TableFormatter::value_cell(format!("  {}  ", " ".on_truecolor(r, g, b)))
    ]);
    println!("{info_table}");

    println!("\n{}", Theme::header("--- Color Palettes ---"));
    
    let mut palette_table = TableFormatter::create_table();
    palette_table.set_header(vec![
        TableFormatter::header_cell("Relation"),
        TableFormatter::header_cell("Hex"),
        TableFormatter::header_cell("HSL"),
        TableFormatter::header_cell("Preview"),
    ]);

    // Complementary
    let comp_h = (h + 180.0) % 360.0;
    add_palette_row(&mut palette_table, "Complementary", comp_h, s, l);

    // Analogous
    add_palette_row(&mut palette_table, "Analogous Left", (h + 330.0) % 360.0, s, l);
    add_palette_row(&mut palette_table, "Analogous Right", (h + 30.0) % 360.0, s, l);

    // Triadic
    add_palette_row(&mut palette_table, "Triadic 1", (h + 120.0) % 360.0, s, l);
    add_palette_row(&mut palette_table, "Triadic 2", (h + 240.0) % 360.0, s, l);

    // Monochromatic
    add_palette_row(&mut palette_table, "Darker", h, s, (l - 0.2).max(0.0));
    add_palette_row(&mut palette_table, "Lighter", h, s, (l + 0.2).min(1.0));

    println!("{palette_table}");
    
    Ok(())
}

fn add_palette_row(table: &mut comfy_table::Table, label: &str, h: f64, s: f64, l: f64) {
    let (r, g, b) = hsl_to_rgb(h, s, l);
    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    table.add_row(vec![
        TableFormatter::value_cell(label),
        TableFormatter::value_cell(hex),
        TableFormatter::value_cell(format!("{:.0}°, {:.0}%, {:.0}%", h, s * 100.0, l * 100.0)),
        TableFormatter::value_cell(format!("  {}  ", " ".on_truecolor(r, g, b))),
    ]);
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let mut h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if h < 0.0 { h += 360.0; }

    let l = (max + min) / 2.0;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    (h, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_p, g_p, b_p) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_p + m) * 255.0).round() as u8,
        ((g_p + m) * 255.0).round() as u8,
        ((b_p + m) * 255.0).round() as u8,
    )
}
