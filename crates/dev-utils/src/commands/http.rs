use anyhow::Result;
use reqwest::{Client, Method};
use serde_json::Value;
use std::str::FromStr;
use std::path::PathBuf;
use std::fs;
use owo_colors::OwoColorize;
use cli_core::output::TableFormatter;

pub async fn request(
    method: String,
    url: String,
    body: Option<String>,
    headers: Vec<String>,
    output: Option<PathBuf>,
    verbose: bool,
) -> Result<()> {
    let client = Client::new();
    let method = Method::from_str(&method.to_uppercase())?;
    
    let mut request = client.request(method.clone(), &url);

    if verbose {
        println!("{} {} {}", "Request:".bold().cyan(), method.to_string().yellow(), url.underline());
    }

    for header in headers {
        if let Some((key, value)) = header.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if verbose {
                println!("  {} {}: {}", "->".dimmed(), key.blue(), value);
            }
            request = request.header(key, value);
        }
    }

    if let Some(body_content) = body {
        request = request.body(body_content);
    }

    let response = request.send().await?;
    let status = response.status();
    
    let status_str = if status.is_success() {
        status.to_string().green().to_string()
    } else {
        status.to_string().red().to_string()
    };
    println!("{} {}", "Status:".bold(), status_str);
    
    if verbose {
        println!("\n{}", "Response Headers:".bold().cyan());
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("Header"),
            TableFormatter::header_cell("Value"),
        ]);
        
        for (key, value) in response.headers().iter() {
            table.add_row(vec![
                TableFormatter::highlight_cell(key.to_string()),
                TableFormatter::value_cell(value.to_str().unwrap_or("[Binary Data]")),
            ]);
        }
        println!("{}", table);
    }
    
    let text = response.text().await?;
    
    if let Some(path) = output {
        fs::write(&path, &text)?;
        println!("{} Body saved to {}", "Success:".green().bold(), path.display().yellow());
    } else {
        println!("\n{}", "Body:".bold());
        if let Ok(json) = serde_json::from_str::<Value>(&text) {
            println!("{}", serde_json::to_string_pretty(&json)?);
        } else {
            println!("{}", text);
        }
    }

    Ok(())
}
