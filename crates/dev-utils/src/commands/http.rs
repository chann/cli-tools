use anyhow::Result;
use reqwest::{Client, Method};
use serde_json::Value;
use std::str::FromStr;
use owo_colors::OwoColorize;

pub async fn request(
    method: String,
    url: String,
    body: Option<String>,
    headers: Vec<String>,
) -> Result<()> {
    let client = Client::new();
    let method = Method::from_str(&method.to_uppercase())?;
    
    let mut request = client.request(method, &url);

    for header in headers {
        if let Some((key, value)) = header.split_once(':') {
            request = request.header(key.trim(), value.trim());
        }
    }

    if let Some(body_content) = body {
        request = request.body(body_content);
    }

    let response = request.send().await?;
    let status = response.status();
    
    println!("{} {}", "Status:".bold(), status.to_string().green());
    
    let headers = response.headers();
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key.to_string().cyan(), value);
    }
    
    println!("\n{}", "Body:".bold());
    let text = response.text().await?;
    
    if let Ok(json) = serde_json::from_str::<Value>(&text) {
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("{}", text);
    }

    Ok(())
}
