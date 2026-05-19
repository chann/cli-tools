use anyhow::Result;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ProjectStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub extension_counts: HashMap<String, (usize, usize)>,
}

pub async fn get_stats(path: &Path) -> Result<ProjectStats> {
    let mut total_files = 0;
    let mut total_lines = 0;
    let mut extension_counts: HashMap<String, (usize, usize)> = HashMap::new();

    let walker = WalkBuilder::new(path)
        .hidden(false)
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
                let lines = content.lines().count();
                let ext = path.extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_else(|| "no extension".to_string());

                total_files += 1;
                total_lines += lines;

                let stats = extension_counts.entry(ext).or_insert((0, 0));
                stats.0 += 1;
                stats.1 += lines;
            }
        }
    }

    Ok(ProjectStats {
        total_files,
        total_lines,
        extension_counts,
    })
}

pub async fn show(path: &Path) -> Result<()> {
    let stats = get_stats(path).await?;
    let total_files = stats.total_files;
    let total_lines = stats.total_lines;
    let extension_counts = stats.extension_counts;

    let mut summary_table = TableFormatter::create_table();
    summary_table.add_row(vec![
        TableFormatter::header_cell("Metric"),
        TableFormatter::header_cell("Value"),
    ]);
    summary_table.add_row(vec![
        TableFormatter::value_cell("Total Files"),
        TableFormatter::highlight_cell(total_files.to_string()),
    ]);
    summary_table.add_row(vec![
        TableFormatter::value_cell("Total Lines"),
        TableFormatter::highlight_cell(total_lines.to_string()),
    ]);
    println!("{}", summary_table);

    println!("\n{}", Theme::header("Language Breakdown"));
    
    let mut sorted_stats: Vec<_> = extension_counts.into_iter().collect();
    sorted_stats.sort_by(|a, b| b.1.1.cmp(&a.1.1)); // Sort by lines descending

    let mut breakdown_table = TableFormatter::create_table();
    breakdown_table.set_header(vec![
        TableFormatter::header_cell("Extension"),
        TableFormatter::header_cell("Files"),
        TableFormatter::header_cell("Lines"),
        TableFormatter::header_cell("Percentage"),
    ]);

    for (ext, (files, lines)) in sorted_stats.iter().take(10) {
        let percentage = if total_lines > 0 {
            (*lines as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };

        breakdown_table.add_row(vec![
            TableFormatter::highlight_cell(ext),
            TableFormatter::value_cell(files.to_string()),
            TableFormatter::value_cell(lines.to_string()),
            TableFormatter::value_cell(format!("{:.1}%", percentage)),
        ]);
    }

    println!("{}", breakdown_table);

    if sorted_stats.len() > 10 {
        println!("  {}", Theme::dim(format!("... and {} more extensions", sorted_stats.len() - 10)));
    }

    Ok(())
}
