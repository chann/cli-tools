use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn convert(value: f64, from: &str, to: &str) -> Result<()> {
    let from = from.to_lowercase();
    let to = to.to_lowercase();

    let result = match (from.as_str(), to.as_str()) {
        // Temperature
        ("c", "f") => Some((value * 1.8 + 32.0, "°F")),
        ("f", "c") => Some(((value - 32.0) / 1.8, "°C")),
        ("c", "k") => Some((value + 273.15, "K")),
        ("k", "c") => Some((value - 273.15, "°C")),
        ("f", "k") => Some(((value - 32.0) / 1.8 + 273.15, "K")),
        ("k", "f") => Some(((value - 273.15) * 1.8 + 32.0, "°F")),

        // Length (base: meter)
        ("m", "km") => Some((value / 1000.0, "km")),
        ("km", "m") => Some((value * 1000.0, "m")),
        ("m", "ft") => Some((value * 3.28084, "ft")),
        ("ft", "m") => Some((value / 3.28084, "m")),
        ("m", "in") => Some((value * 39.3701, "in")),
        ("in", "m") => Some((value / 39.3701, "m")),
        ("m", "cm") => Some((value * 100.0, "cm")),
        ("cm", "m") => Some((value / 100.0, "m")),
        ("m", "mm") => Some((value * 1000.0, "mm")),
        ("mm", "m") => Some((value / 1000.0, "m")),
        ("mi", "km") => Some((value * 1.60934, "km")),
        ("km", "mi") => Some((value / 1.60934, "mi")),

        // Weight (base: kg)
        ("kg", "lb") => Some((value * 2.20462, "lb")),
        ("lb", "kg") => Some((value / 2.20462, "kg")),
        ("kg", "g") => Some((value * 1000.0, "g")),
        ("g", "kg") => Some((value / 1000.0, "kg")),
        ("lb", "oz") => Some((value * 16.0, "oz")),
        ("oz", "lb") => Some((value / 16.0, "lb")),

        _ => None,
    };

    if let Some((res, unit)) = result {
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("From"),
            TableFormatter::header_cell("To"),
            TableFormatter::header_cell("Result"),
        ]);

        table.add_row(vec![
            TableFormatter::value_cell(format!("{} {}", value, from)),
            TableFormatter::value_cell(to),
            TableFormatter::highlight_cell(format!("{:.4} {}", res, unit)),
        ]);

        println!("{}", Theme::header("Unit Conversion"));
        println!("{}", table);
    } else {
        println!("{}", Theme::error(format!("Unsupported conversion: {} to {}", from, to)));
        println!("{}", Theme::info("Supported units:"));
        println!("  - Temperature: c, f, k");
        println!("  - Length: m, km, mi, ft, in, cm, mm");
        println!("  - Weight: kg, g, lb, oz");
    }

    Ok(())
}
