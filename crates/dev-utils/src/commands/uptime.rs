use anyhow::Result;
use reqwest::Client;
use std::time::Instant;
use owo_colors::OwoColorize;

pub async fn check_multiple(urls: Vec<String>) -> Result<()> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    if urls.is_empty() {
        anyhow::bail!("No URLs provided to check");
    }

    for url in urls {
        check(&client, &url).await?;
        println!(); // Add a newline between checks
    }

    Ok(())
}

async fn check(client: &Client, url: &str) -> Result<()> {
    let mut target_url = url.to_string();
    if !target_url.starts_with("http://") && !target_url.starts_with("https://") {
        target_url = format!("https://{}", target_url);
    }

    println!("Checking uptime for {}...", target_url.cyan());

    let start = Instant::now();
    let response = client.get(&target_url).send().await;
    let duration = start.elapsed();

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                println!(
                    "{} Status: {} ({}), Response Time: {:?}",
                    "UP".green().bold(),
                    status.as_u16().green(),
                    status.canonical_reason().unwrap_or("Unknown"),
                    duration
                );
            } else {
                println!(
                    "{} Status: {} ({}), Response Time: {:?}",
                    "DOWN/ISSUE".yellow().bold(),
                    status.as_u16().yellow(),
                    status.canonical_reason().unwrap_or("Unknown"),
                    duration
                );
            }
        }
        Err(e) => {
            println!(
                "{} Error: {}",
                "DOWN".red().bold(),
                e.to_string().red()
            );
        }
    }

    Ok(())
}
