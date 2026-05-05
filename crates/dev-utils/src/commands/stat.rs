use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use std::path::Path;
use chrono::{DateTime, Local};

pub fn analyze(input: &str, is_file: bool) -> Result<()> {
    if is_file {
        analyze_file(input)
    } else {
        analyze_string(input)
    }
}

fn analyze_string(text: &str) -> Result<()> {
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Metric"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Characters"),
        TableFormatter::highlight_cell(text.len()),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Characters (no space)"),
        TableFormatter::value_cell(text.chars().filter(|c| !c.is_whitespace()).count()),
    ]);

    let words: Vec<&str> = text.split_whitespace().collect();
    table.add_row(vec![
        TableFormatter::value_cell("Words"),
        TableFormatter::highlight_cell(words.len()),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Lines"),
        TableFormatter::highlight_cell(text.lines().count()),
    ]);

    if !words.is_empty() {
        let avg_len = words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64;
        table.add_row(vec![
            TableFormatter::value_cell("Avg. Word Length"),
            TableFormatter::value_cell(format!("{:.2}", avg_len)),
        ]);
    }

    println!("{}", Theme::header("Text Statistics"));
    println!("{}", table);
    Ok(())
}

fn analyze_file(path_str: &str) -> Result<()> {
    let path = Path::new(path_str);
    let metadata = std::fs::metadata(path)?;

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Property"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Path"),
        TableFormatter::value_cell(path_str),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Size"),
        TableFormatter::highlight_cell(format!("{} bytes", metadata.len())),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Type"),
        if metadata.is_dir() {
            TableFormatter::value_cell("Directory")
        } else if metadata.is_file() {
            TableFormatter::value_cell("File")
        } else if metadata.is_symlink() {
            TableFormatter::value_cell("Symlink")
        } else {
            TableFormatter::value_cell("Other")
        },
    ]);

    if let Ok(created) = metadata.created() {
        let dt: DateTime<Local> = created.into();
        table.add_row(vec![
            TableFormatter::value_cell("Created"),
            TableFormatter::value_cell(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        ]);
    }

    if let Ok(modified) = metadata.modified() {
        let dt: DateTime<Local> = modified.into();
        table.add_row(vec![
            TableFormatter::value_cell("Modified"),
            TableFormatter::value_cell(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        ]);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        table.add_row(vec![
            TableFormatter::value_cell("Permissions"),
            TableFormatter::value_cell(format!("{:o}", metadata.mode() & 0o777)),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Owner (UID)"),
            TableFormatter::value_cell(metadata.uid()),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Group (GID)"),
            TableFormatter::value_cell(metadata.gid()),
        ]);
    }

    println!("{}", Theme::header("File Statistics"));
    println!("{}", table);
    Ok(())
}
