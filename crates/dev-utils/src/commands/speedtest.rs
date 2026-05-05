use anyhow::Result;
use reqwest::Client;
use std::time::Instant;
use owo_colors::OwoColorize;
use futures_util::StreamExt;
use std::io::{stdout, Write};

pub async fn run() -> Result<()> {
    println!("{}", "--- Network Speed Test ---".bold().cyan());
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // 1. Latency Test (Ping)
    print!("Measuring latency... ");
    stdout().flush()?;
    let mut latencies = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let _ = client.get("https://speed.cloudflare.com/meta").send().await?;
        latencies.push(start.elapsed().as_millis());
    }
    let avg_latency = latencies.iter().sum::<u128>() as f64 / latencies.len() as f64;
    println!("{:.2} ms", avg_latency.bold().green());

    // 2. Download Test
    println!("\n{}", "Testing download speed...".bold().yellow());
    let download_url = "https://speed.cloudflare.com/__down?bytes=25000000"; // 25MB
    let start = Instant::now();
    let response = client.get(download_url).send().await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Failed to connect to download test server");
    }

    let mut downloaded = 0;
    let total_download_size = response.content_length().unwrap_or(25_000_000);
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        downloaded += chunk.len();
        
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            let speed = (downloaded as f64 / 1024.0 / 1024.0) / elapsed;
            print!("\r  Progress: {:>3}% | Speed: {:.2} MB/s", 
                (downloaded * 100 / total_download_size as usize),
                speed.green());
            stdout().flush()?;
        }
    }
    let total_download_elapsed = start.elapsed().as_secs_f64();
    let final_download_speed_mbs = (downloaded as f64 / 1024.0 / 1024.0) / total_download_elapsed;

    // 3. Upload Test
    println!("\n\n{}", "Testing upload speed...".bold().magenta());
    let upload_url = "https://speed.cloudflare.com/__up";
    let upload_size = 10_000_000; // 10MB
    let upload_data = vec![0u8; upload_size];
    
    let start = Instant::now();
    let response = client.post(upload_url)
        .body(upload_data)
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to connect to upload test server");
    }

    let total_upload_elapsed = start.elapsed().as_secs_f64();
    let final_upload_speed_mbs = (upload_size as f64 / 1024.0 / 1024.0) / total_upload_elapsed;
    println!("  Progress: 100% | Speed: {:.2} MB/s", final_upload_speed_mbs.green());

    // Final Result
    println!("\n{}", "--- Result Summary ---".bold().bright_white().on_black());
    println!("{:<15}: {:.2} ms", "Latency (Ping)", avg_latency.green());
    println!("{:<15}: {:.2} MB/s ({:.2} Mbps)", 
        "Download", 
        final_download_speed_mbs.bold().green(),
        (final_download_speed_mbs * 8.0).bold().green());
    println!("{:<15}: {:.2} MB/s ({:.2} Mbps)", 
        "Upload", 
        final_upload_speed_mbs.bold().green(),
        (final_upload_speed_mbs * 8.0).bold().green());

    Ok(())
}
