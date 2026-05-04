use anyhow::Result;
use reqwest::Client;
use std::time::Instant;
use owo_colors::OwoColorize;

pub async fn run() -> Result<()> {
    println!("{}", "Starting download speed test...".bold().cyan());
    
    let url = "https://speed.cloudflare.com/__down?bytes=25000000"; // 25MB
    let client = Client::new();
    
    let start = Instant::now();
    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Failed to connect to speed test server");
    }

    let mut downloaded = 0;
    let total_size = response.content_length().unwrap_or(25_000_000);
    
    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        downloaded += chunk.len();
        
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            let speed = (downloaded as f64 / 1024.0 / 1024.0) / elapsed;
            print!("\rDownloaded: {}/{} MB | Speed: {:.2} MB/s", 
                downloaded / 1024 / 1024, 
                total_size / 1024 / 1024,
                speed.green());
            use std::io::{stdout, Write};
            stdout().flush()?;
        }
    }

    let total_elapsed = start.elapsed().as_secs_f64();
    let final_speed_mbps = (downloaded as f64 * 8.0 / 1_000_000.0) / total_elapsed;
    let final_speed_mbs = (downloaded as f64 / 1024.0 / 1024.0) / total_elapsed;

    println!("\n\n{}", "--- Result ---".bold().yellow());
    println!("Total Downloaded: {} MB", downloaded / 1024 / 1024);
    println!("Total Time: {:.2}s", total_elapsed);
    println!("Average Speed: {:.2} MB/s ({:.2} Mbps)", 
        final_speed_mbs.bold().green(), 
        final_speed_mbps.bold().green());

    Ok(())
}
