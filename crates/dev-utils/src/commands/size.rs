use anyhow::Result;
use owo_colors::OwoColorize;

pub fn convert(value: f64, from_unit: &str) -> Result<()> {
    let bytes = match from_unit.to_lowercase().as_str() {
        "b" | "byte" | "bytes" => value,
        "k" | "kb" | "kib" => value * 1024.0,
        "m" | "mb" | "mib" => value * 1024.0 * 1024.0,
        "g" | "gb" | "gib" => value * 1024.0 * 1024.0 * 1024.0,
        "t" | "tb" | "tib" => value * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => anyhow::bail!("Unsupported unit: {}. Use B, KB, MB, GB, or TB.", from_unit),
    };

    println!("{}", format!("--- Conversions for {} {} ---", value, from_unit).bold().cyan());
    
    println!("{:<10}: {:>20.0} B", "Bytes", bytes);
    println!("{:<10}: {:>20.2} KB", "Kilobytes", bytes / 1024.0);
    println!("{:<10}: {:>20.2} MB", "Megabytes", bytes / (1024.0 * 1024.0));
    println!("{:<10}: {:>20.2} GB", "Gigabytes", bytes / (1024.0 * 1024.0 * 1024.0));
    println!("{:<10}: {:>20.2} TB", "Terabytes", bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0));

    Ok(())
}
