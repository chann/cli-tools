use anyhow::{Result, anyhow};
use std::process::Command;
use serde_json::Value;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn open(issue: Option<u32>, pr: Option<u32>) -> Result<()> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .map_err(|e| anyhow!("Failed to execute git command: {}", e))?;

    if !output.status.success() {
        anyhow::bail!("Failed to get git remote URL. Are you in a git repository with an 'origin' remote?");
    }

    let mut url = String::from_utf8(output.stdout)?.trim().to_string();

    // Convert SSH URL to HTTPS URL
    if url.starts_with("git@") {
        url = url.replace(":", "/").replace("git@", "https://");
    }
    if url.ends_with(".git") {
        url.truncate(url.len() - 4);
    }

    if let Some(issue_num) = issue {
        url = format!("{}/issues/{}", url, issue_num);
    } else if let Some(pr_num) = pr {
        url = format!("{}/pull/{}", url, pr_num);
    }

    println!("{}", Theme::info(format!("Opening {}...", url)));

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "start", &url]).spawn()?;
    }

    Ok(())
}

pub async fn get_user(username: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://api.github.com/users/{}", username))
        .header("User-Agent", "dev-tools")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to fetch user info: {}", res.status());
    }

    let user: Value = res.json().await?;

    println!("{}", Theme::header(format!("GitHub User: {}", username)));
    
    let mut table = TableFormatter::create_table();
    table.add_row(vec![TableFormatter::header_cell("Field"), TableFormatter::header_cell("Value")]);
    table.add_row(vec![TableFormatter::value_cell("Name"), TableFormatter::highlight_cell(user["name"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("Bio"), TableFormatter::value_cell(user["bio"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("Location"), TableFormatter::value_cell(user["location"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("Public Repos"), TableFormatter::value_cell(user["public_repos"].to_string())]);
    table.add_row(vec![TableFormatter::value_cell("Followers"), TableFormatter::value_cell(user["followers"].to_string())]);
    table.add_row(vec![TableFormatter::value_cell("Following"), TableFormatter::value_cell(user["following"].to_string())]);
    table.add_row(vec![TableFormatter::value_cell("URL"), TableFormatter::value_cell(user["html_url"].as_str().unwrap_or("N/A"))]);

    println!("{}", table);

    Ok(())
}

pub async fn get_repo(repo_path: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://api.github.com/repos/{}", repo_path))
        .header("User-Agent", "dev-tools")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to fetch repo info: {}", res.status());
    }

    let repo: Value = res.json().await?;

    println!("{}", Theme::header(format!("GitHub Repo: {}", repo_path)));
    
    let mut table = TableFormatter::create_table();
    table.add_row(vec![TableFormatter::header_cell("Field"), TableFormatter::header_cell("Value")]);
    table.add_row(vec![TableFormatter::value_cell("Full Name"), TableFormatter::highlight_cell(repo["full_name"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("Description"), TableFormatter::value_cell(repo["description"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("Language"), TableFormatter::value_cell(repo["language"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("Stars"), TableFormatter::value_cell(repo["stargazers_count"].to_string())]);
    table.add_row(vec![TableFormatter::value_cell("Forks"), TableFormatter::value_cell(repo["forks_count"].to_string())]);
    table.add_row(vec![TableFormatter::value_cell("Open Issues"), TableFormatter::value_cell(repo["open_issues_count"].to_string())]);
    table.add_row(vec![TableFormatter::value_cell("License"), TableFormatter::value_cell(repo["license"]["name"].as_str().unwrap_or("N/A"))]);
    table.add_row(vec![TableFormatter::value_cell("URL"), TableFormatter::value_cell(repo["html_url"].as_str().unwrap_or("N/A"))]);

    println!("{}", table);

    Ok(())
}

pub async fn search(query: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.github.com/search/repositories")
        .query(&[("q", query), ("sort", "stars"), ("order", "desc"), ("per_page", "10")])
        .header("User-Agent", "dev-tools")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to search repositories: {}", res.status());
    }

    let search_results: Value = res.json().await?;
    let items = search_results["items"].as_array().ok_or_else(|| anyhow!("Invalid response from GitHub"))?;

    println!("{}", Theme::info(format!("GitHub Repository Search: {}", query)));

    if items.is_empty() {
        println!("{}", Theme::warning("No repositories found."));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Repository"),
        TableFormatter::header_cell("Description"),
        TableFormatter::header_cell("Stars"),
        TableFormatter::header_cell("Language")
    ]);

    for item in items {
        let name = item["full_name"].as_str().unwrap_or("N/A");
        let desc = item["description"].as_str().unwrap_or("N/A");
        let desc_trimmed = if desc.chars().count() > 60 {
            format!("{}...", desc.chars().take(57).collect::<String>())
        } else {
            desc.to_string()
        };
        let stars = item["stargazers_count"].as_u64().unwrap_or(0);
        let lang = item["language"].as_str().unwrap_or("N/A");

        table.add_row(vec![
            TableFormatter::highlight_cell(name),
            TableFormatter::value_cell(desc_trimmed),
            TableFormatter::value_cell(stars.to_string()),
            TableFormatter::value_cell(lang),
        ]);
    }

    println!("\n{table}");

    Ok(())
}
