use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use owo_colors::OwoColorize;

#[derive(Deserialize, Debug)]
struct CrateResponse {
    crates: Vec<CrateInfo>,
}

#[derive(Deserialize, Debug)]
struct CrateInfo {
    name: String,
    max_version: String,
    description: Option<String>,
    downloads: u64,
}

pub async fn search(query: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("dev-utils (https://github.com/chann/cli-tools)"));

    let url = format!("https://crates.io/api/v1/crates?page=1&per_page=10&q={}", query);
    
    println!("Searching for crates matching {}...", query.cyan());

    let response = client.get(url)
        .headers(headers)
        .send()
        .await?
        .json::<CrateResponse>()
        .await?;

    if response.crates.is_empty() {
        println!("No crates found for query: {}", query.red());
        return Ok(());
    }

    println!("\n{:<20} {:<10} {:<10} {:<}", 
        "Name".bold(), "Version".bold(), "Downloads".bold(), "Description".bold());
    println!("{}", "-".repeat(80).dimmed());

    for c in response.crates {
        let desc = c.description.unwrap_or_default();
        let desc_trimmed = if desc.len() > 50 {
            format!("{}...", &desc[..47])
        } else {
            desc
        };

        println!("{:<20} {:<10} {:<10} {:<}", 
            c.name.green(), 
            c.max_version.yellow(), 
            c.downloads.to_string().blue(), 
            desc_trimmed);
    }

    Ok(())
}
