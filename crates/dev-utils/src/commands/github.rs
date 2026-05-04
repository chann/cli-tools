use anyhow::{Result, anyhow};
use std::process::Command;
use serde_json::Value;
use owo_colors::OwoColorize;

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

    println!("Opening {}...", url);

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
        .header("User-Agent", "dev-utils")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to fetch user info: {}", res.status());
    }

    let user: Value = res.json().await?;

    println!("{}", format!("--- GitHub User: {} ---", username).bold().cyan());
    println!("{:<15}: {}", "Name", user["name"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "Bio", user["bio"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "Location", user["location"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "Public Repos", user["public_repos"]);
    println!("{:<15}: {}", "Followers", user["followers"]);
    println!("{:<15}: {}", "Following", user["following"]);
    println!("{:<15}: {}", "URL", user["html_url"].as_str().unwrap_or("N/A"));

    Ok(())
}

pub async fn get_repo(repo_path: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://api.github.com/repos/{}", repo_path))
        .header("User-Agent", "dev-utils")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("Failed to fetch repo info: {}", res.status());
    }

    let repo: Value = res.json().await?;

    println!("{}", format!("--- GitHub Repo: {} ---", repo_path).bold().green());
    println!("{:<15}: {}", "Full Name", repo["full_name"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "Description", repo["description"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "Language", repo["language"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "Stars", repo["stargazers_count"]);
    println!("{:<15}: {}", "Forks", repo["forks_count"]);
    println!("{:<15}: {}", "Open Issues", repo["open_issues_count"]);
    println!("{:<15}: {}", "License", repo["license"]["name"].as_str().unwrap_or("N/A"));
    println!("{:<15}: {}", "URL", repo["html_url"].as_str().unwrap_or("N/A"));

    Ok(())
}
