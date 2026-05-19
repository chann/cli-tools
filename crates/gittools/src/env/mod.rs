use anyhow::{Context, Result};
use cli_core::ui::Theme;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn check(path: &Path) -> Result<()> {
    let env_file = path.join(".env");
    let example_file = path.join(".env.example");

    if !example_file.exists() {
        println!(
            "{} .env.example not found. Skipping validation.",
            Theme::dim("Skipped:")
        );
        return Ok(());
    }

    let example_keys = extract_keys(&example_file)?;
    if example_keys.is_empty() {
        println!(
            "{} No keys found in .env.example.",
            Theme::info("Note:")
        );
        return Ok(());
    }

    if !env_file.exists() {
        println!(
            "{} .env file is missing! Found {} keys in .env.example that should be configured.",
            Theme::error("Error:"),
            example_keys.len()
        );
        for key in example_keys {
            println!("  • {}", key);
        }
        return Ok(());
    }

    let env_keys = extract_keys(&env_file)?;
    let mut missing_keys = Vec::new();

    for key in &example_keys {
        if !env_keys.contains(key) {
            missing_keys.push(key);
        }
    }

    if missing_keys.is_empty() {
        println!(
            "{} All {} keys from .env.example are present in .env",
            Theme::success("Success:"),
            example_keys.len()
        );
    } else {
        println!(
            "{} Found {} missing keys in .env:",
            Theme::warning("Warning:"),
            missing_keys.len()
        );
        for key in missing_keys {
            println!("  • {}", key);
        }
        println!();
        println!(
            "{}",
            Theme::dim("Tip: Update your .env file with these keys to ensure the application runs correctly.")
        );
    }

    Ok(())
}

fn extract_keys(path: &Path) -> Result<HashSet<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    let mut keys = HashSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(key) = line.split('=').next() {
            let key = key.trim();
            if !key.is_empty() {
                keys.insert(key.to_string());
            }
        }
    }
    Ok(keys)
}
