use anyhow::Result;
use owo_colors::OwoColorize;

pub fn convert(value: f64, from: &str, to: &str) -> Result<()> {
    let from = from.to_lowercase();
    let to = to.to_lowercase();

    let result = match (from.as_str(), to.as_str()) {
        // Temperature
        ("c", "f") => value * 1.8 + 32.0,
        ("f", "c") => (value - 32.0) / 1.8,
        ("c", "k") => value + 273.15,
        ("k", "c") => value - 273.15,
        ("f", "k") => (value - 32.0) / 1.8 + 273.15,
        ("k", "f") => (value - 273.15) * 1.8 + 32.0,

        // Length
        ("m", "ft") => value * 3.28084,
        ("ft", "m") => value / 3.28084,
        ("km", "mi") => value * 0.621371,
        ("mi", "km") => value / 0.621371,
        ("cm", "in") => value * 0.393701,
        ("in", "cm") => value / 0.393701,
        ("m", "yd") => value * 1.09361,
        ("yd", "m") => value / 1.09361,

        // Weight
        ("kg", "lb") => value * 2.20462,
        ("lb", "kg") => value / 2.20462,
        ("g", "oz") => value * 0.035274,
        ("oz", "g") => value / 0.035274,

        _ => anyhow::bail!("Unsupported conversion from {} to {}", from, to),
    };

    println!(
        "{} {} {} = {} {}",
        value.cyan(),
        from.bold(),
        "is".dimmed(),
        format!("{:.4}", result).green().bold(),
        to.bold()
    );

    Ok(())
}
