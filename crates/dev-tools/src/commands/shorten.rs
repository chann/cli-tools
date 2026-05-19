use anyhow::Result;
use reqwest::Client;
use cli_core::ui::Theme;

pub async fn shorten(url: &str) -> Result<()> {
    println!("{}", Theme::info(format!("Shortening URL: {}", url)));
    
    let client = Client::new();
    
    // Using TinyURL API
    let resp = client
        .get(format!("https://tinyurl.com/api-create.php?url={}", url))
        .send()
        .await?;

    if resp.status().is_success() {
        let short_url = resp.text().await?;
        println!("\n{}", Theme::header("Shortened URL"));
        println!("{}", Theme::highlight(&short_url));
    } else {
        anyhow::bail!("Failed to shorten URL: {}", resp.status());
    }

    Ok(())
}
