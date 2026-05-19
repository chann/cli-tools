use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

#[derive(Deserialize, Debug)]
struct CrateResponse {
    crates: Vec<CrateInfo>,
}

#[derive(Deserialize, Debug)]
struct CrateInfo {
    name: String,
    max_version: String,
    description: Option<String>,
    downloads: u64,
}

pub async fn search(query: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("devtools (https://github.com/chann/cli-tools)"));

    let url = format!("https://crates.io/api/v1/crates?page=1&per_page=10&q={}", query);
    
    println!("{}", Theme::info(format!("Searching for crates matching '{}'...", query)));

    let response = client.get(url)
        .headers(headers)
        .send()
        .await?
        .json::<CrateResponse>()
        .await?;

    if response.crates.is_empty() {
        println!("{}", Theme::warning(format!("No crates found for query: {}", query)));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Name"),
        TableFormatter::header_cell("Version"),
        TableFormatter::header_cell("Downloads"),
        TableFormatter::header_cell("Description"),
    ]);

    for c in response.crates {
        let desc = c.description.unwrap_or_default();
        let desc_trimmed = if desc.len() > 60 {
            format!("{}...", &desc[..57])
        } else {
            desc
        };

        table.add_row(vec![
            TableFormatter::highlight_cell(&c.name),
            TableFormatter::value_cell(&c.max_version),
            TableFormatter::value_cell(format_downloads(c.downloads)),
            TableFormatter::value_cell(&desc_trimmed),
        ]);
    }

    println!("\n{}", table);

    Ok(())
}

fn format_downloads(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}
