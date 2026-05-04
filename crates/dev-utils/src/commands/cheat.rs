use anyhow::Result;

pub async fn run(query: &str) -> Result<()> {
    let url = format!("https://cheat.sh/{}", query);
    
    // cheat.sh detects curl/httpie and returns plain text with ANSI colors
    // We want to mimic this behavior
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header("User-Agent", "curl/7.88.1")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to fetch cheat sheet: {}", res.status());
    }

    let body = res.text().await?;
    println!("{}", body);

    Ok(())
}
