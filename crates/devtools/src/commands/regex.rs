use regex::Regex;
use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;

pub fn test(pattern: &str, text: &str) -> Result<()> {
    let re = Regex::new(pattern).map_err(|e| anyhow!("Invalid regex: {}", e))?;
    
    if re.is_match(text) {
        println!("{}", "Match found!".green().bold());
        
        for (i, cap) in re.captures_iter(text).enumerate() {
            println!("\n{}", format!("--- Capture Group {} ---", i + 1).cyan());
            for (j, name) in re.capture_names().enumerate() {
                if let Some(m) = cap.get(j) {
                    let default_name = j.to_string();
                    let label = name.unwrap_or(&default_name);
                    println!("{:<10}: {}", label, m.as_str().yellow());
                }
            }
        }
    } else {
        println!("{}", "No match found.".red());
    }
    
    Ok(())
}
