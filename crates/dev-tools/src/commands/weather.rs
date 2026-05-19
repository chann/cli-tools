use anyhow::Result;
use cli_core::ui::Theme;

pub async fn get_weather(location: Option<String>) -> Result<()> {
    let url = match location {
        Some(loc) => format!("https://wttr.in/{}?0", loc),
        None => "https://wttr.in/?0".to_string(),
    };

    println!("{}", Theme::info("Fetching weather information..."));

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "curl/7.64.1") // wttr.in returns terminal-friendly output for curl
        .send()
        .await?;

    if resp.status().is_success() {
        let text = resp.text().await?;
        println!("{}", text);
    } else {
        anyhow::bail!("Failed to fetch weather: {}", resp.status());
    }

    Ok(())
}
