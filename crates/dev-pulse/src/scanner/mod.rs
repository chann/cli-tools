use anyhow::Result;
use cli_core::ui::Theme;
use ignore::WalkBuilder;
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Marker {
    kind: String,
    content: String,
    line: usize,
    file: String,
}

pub async fn scan(path: &Path, markers: Option<Vec<String>>, hidden: bool) -> Result<()> {
    let default_markers = vec![
        "TODO".to_string(),
        "FIXME".to_string(),
        "BUG".to_string(),
        "HACK".to_string(),
        "OPTIMIZE".to_string(),
    ];
    let active_markers = markers.unwrap_or(default_markers);

    // Create a regex to match markers
    let pattern = format!(r"(?i)\b({})\b:?\s*(.*)", active_markers.join("|"));
    let re = Regex::new(&pattern)?;

    println!(
        "{} Scanning for markers: {}",
        Theme::info("Info:"),
        Theme::highlight(&active_markers.join(", "))
    );

    let mut found_markers = Vec::new();

    let walker = WalkBuilder::new(path)
        .hidden(!hidden)
        .git_ignore(true)
        .build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            let path = entry.path();
            if let Ok(content) = fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if let Some(caps) = re.captures(line) {
                        let kind = caps.get(1).unwrap().as_str().to_uppercase();
                        let text = caps.get(2).map_or("", |m| m.as_str().trim());
                        
                        found_markers.push(Marker {
                            kind,
                            content: text.to_string(),
                            line: i + 1,
                            file: entry.path().to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }
    }

    if found_markers.is_empty() {
        println!("{}", Theme::success("No markers found. Your code is clean!"));
        return Ok(());
    }

    println!(
        "{} Found {} markers:",
        Theme::warning("Found:"),
        found_markers.len()
    );
    println!();

    for marker in found_markers {
        println!(
            "  {} {} {} {}",
            Theme::highlight(&format!("[{}]", marker.kind)),
            Theme::dim(&format!("{}:{}", marker.file, marker.line)),
            Theme::dim("-"),
            marker.content
        );
    }

    Ok(())
}
