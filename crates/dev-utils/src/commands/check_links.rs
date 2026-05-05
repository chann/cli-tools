use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use comfy_table::Cell;
use futures_util::stream::{self, StreamExt};
use reqwest::Client;
use std::time::Duration;

pub async fn run(path: &Path) -> Result<()> {
    println!("{}", Theme::info(format!("Searching for links in: {} ...", path.display())));

    let mut files = Vec::new();
    if path.is_file() {
        files.push(path.to_path_buf());
    } else {
        for result in WalkBuilder::new(path).build() {
            if let Ok(entry) = result {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    let url_regex = Regex::new(r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)")?;
    
    let mut url_to_files: std::collections::HashMap<String, Vec<PathBuf>> = std::collections::HashMap::new();

    for file_path in files {
        if let Ok(content) = fs::read_to_string(&file_path) {
            for m in url_regex.find_iter(&content) {
                url_to_files.entry(m.as_str().to_string()).or_default().push(file_path.clone());
            }
        }
    }

    let urls: Vec<String> = url_to_files.keys().cloned().collect();
    if urls.is_empty() {
        println!("{}", Theme::success("No links found."));
        return Ok(());
    }

    println!("{}", Theme::info(format!("Checking {} unique links...", urls.len())));

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Gemini CLI)")
        .build()?;

    let concurrency_limit = 20;
    let mut results = Vec::new();

    let mut stream = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let res = client.head(&url).send().await;
                match res {
                    Ok(resp) => (url, resp.status().is_success(), Some(resp.status().to_string())),
                    Err(e) => (url, false, Some(e.to_string())),
                }
            }
        })
        .buffer_unordered(concurrency_limit);

    while let Some((url, success, status)) = stream.next().await {
        results.push((url, success, status));
    }

    results.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("URL"),
        TableFormatter::header_cell("Status"),
        TableFormatter::header_cell("Found In"),
    ]);

    let mut broken_count = 0;
    for (url, success, status) in results {
        if !success {
            broken_count += 1;
        }

        let status_cell = if success {
            Cell::new(status.unwrap_or_else(|| "OK".to_string())).fg(comfy_table::Color::Green)
        } else {
            Cell::new(status.unwrap_or_else(|| "FAILED".to_string())).fg(comfy_table::Color::Red).add_attribute(comfy_table::Attribute::Bold)
        };

        let files_str = url_to_files.get(&url).unwrap()
            .iter()
            .take(3)
            .map(|p| p.file_name().and_then(|n| n.to_str()).unwrap_or("?"))
            .collect::<Vec<_>>()
            .join(", ");
        
        let files_display = if url_to_files.get(&url).unwrap().len() > 3 {
            format!("{} (+{} more)", files_str, url_to_files.get(&url).unwrap().len() - 3)
        } else {
            files_str
        };

        table.add_row(vec![
            TableFormatter::value_cell(url),
            status_cell,
            TableFormatter::value_cell(files_display),
        ]);
    }

    println!("\n{}", table);

    if broken_count > 0 {
        println!("\n{}", Theme::error(format!("Found {} broken links.", broken_count)));
    } else {
        println!("\n{}", Theme::success("All links are accessible."));
    }

    Ok(())
}
