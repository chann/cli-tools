use anyhow::Result;
use reqwest::Client;

pub async fn shorten(url: &str) -> Result<()> {
    let client = Client::new();
    
    // Using TinyURL API (unauthenticated simple endpoint if available, or just the public one)
    // For TinyURL v1 (public, no key)
    let resp = client
        .get(format!("https://tinyurl.com/api-create.php?url={}", url))
        .send()
        .await?;

    if resp.status().is_success() {
        let short_url = resp.text().await?;
        println!("{}", short_url);
    } else {
        anyhow::bail!("Failed to shorten URL: {}", resp.status());
    }

    Ok(())
}
