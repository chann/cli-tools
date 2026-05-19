use anyhow::Result;
use reqwest;
use owo_colors::OwoColorize;

const API_BASE: &str = "https://www.toptal.com/developers/gitignore/api";

pub async fn list() -> Result<()> {
    let url = format!("{}/list?format=lines", API_BASE);
    let response = reqwest::get(url).await?.text().await?;
    
    println!("{}", "Available Gitignore Templates:".bold().green());
    let mut items: Vec<&str> = response.split('\n').collect();
    items.sort();
    
    // Print in columns
    let term_width = 80; // Default
    let mut current_line = String::new();
    for item in items {
        if item.is_empty() { continue; }
        if current_line.len() + item.len() + 2 > term_width {
            println!("{}", current_line);
            current_line = String::new();
        }
        current_line.push_str(&format!("{: <20}", item));
    }
    if !current_line.is_empty() {
        println!("{}", current_line);
    }

    Ok(())
}

pub async fn generate(targets: Vec<String>) -> Result<()> {
    if targets.is_empty() {
        anyhow::bail!("Please specify at least one target (e.g., rust, macos)");
    }

    let url = format!("{}/{}", API_BASE, targets.join(","));
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch gitignore for: {}. Status: {}", targets.join(", "), response.status());
    }

    let content = response.text().await?;
    println!("{}", content);

    Ok(())
}
